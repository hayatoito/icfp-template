repo=icfp2024
old_repo=icfp2023
site='https://api.icfpcontest.com'
bin=./target/release/$repo
curl_options=(--silent --show-error)
max_problem_id=90

# For gnuplot in xpra
export GNUTERM=wxt

# * Setup

init_from_last_year_repo() {
  cd ~/src
  gh repo create $repo --private --clone

  cd ~/src/$old_repo
  git archive --format 'tar' HEAD | tar -C ~/src/icfp2024 -xvf -

  cd ~/src/$repo

  rustup update

  cargo upgrade --imcompatible
  carog build

  git ls-files | xargs sd $old_repo $repo

  init_dirs
}

init_dirs() {
  mkdir -p {problem,solution,plot,render}
  mkdir -p solution/submission
}

api_token() {
  my-keyring $repo
}

# * Problems

get_number_of_problems() {
  # In-Contest:
  # curl $curl_options $site/problems | jq .number_of_problems

  # Post-Contest
  echo 90
}

download_problems() {
  local num=$(get_number_of_problems)
  echo "problems: $num"

  cd ./problem
  local next=1
  for i in {$next..$num}; do
    curl -O "https://cdn.icfpcontest.com/problems/$i.json"
    # curl "$site/problem?problem_id=$i" -o raw-$i.json
    # cat raw-$i.json | jq -r .Success > $i.json
  done
}

# * Plot

plot_problems() {
  local problems_data=./plot/problems.data
  echo "id musicians attendees" > $problems_data
  for i in {1..$max_problem_id}; do
    echo -n "$i " >> $problems_data
    cat ./problem/$i.json | \
      jq -r '"\(.musicians | length) \(.attendees | length)"' >> $problems_data
  done
  cd ./plot && gnuplot -p ./problems.gnuplot
}

plot_update_score() {
  # Create ./plot/score-{name}.data file from ./solution/{name}/*.json files

  build
  local d=${1:-./solution/best}
  local score_data=./plot/score-${d:t}.data
  echo "id score" > $score_data
  for i in {1..$(get_number_of_problems)}; do
    local score=$($bin score $i $d/$i.json)
    echo "$i $score" >> $score_data
  done
  ls -l $score_data
}

plot_score() {
  # e.g.
  # % mm plot_stats_score ./plot/score-best.data ./plot/score-other.data
  gnuplot -p -e "arg_score1='$1'; arg_score2='$2'" ./plot/score.gnuplot
}

plot_sa() {
  # e.g.
  # % mm plot_sa ./plot/sa/sa-temp0-100-iter-50000/60.data
  gnuplot -p -e "arg_data='$1'" ./plot/sa.gnuplot
}

# * Render

render_problems() {
  build
  for i in {1..$max_problem_id}; do
    $bin plot-problem $i ./render/problem/$i.svg
  done
}

render_solutions() {
  build
  local d=${1:-./solution/best}
  mkdir -p ./render/${d:t}
  for i in {1..$max_problem_id}; do
    $bin render-solution $i $d/$i.json ./render/${d:t}/$i.svg
  done
}

# * Solve

build() {
  cargo build --release
}

solve() {
  build
  time RUST_LOG=info $bin solve $@
}

solve_debug() {
  build
  time RUST_LOG=debug $bin solve ${1:-1}
}

solve_all() {
  build
  for i in {1..$max_problem_id}; do
    time RUST_LOG=info $bin solve ${i}
  done
}

solve_all_parallel() {
  build
  RUST_LOG=info parallel --joblog ./log/joblog --results ./log/results $bin solve {} --initial-solution-path ./solution/best/{}.json ::: {1..$max_problem_id}
}

solve_all_parallel_retry() {
  build
  RUST_LOG=info LANG=C parallel --retry-failed --joblog ./log/joblog
}

solve_basic() {
  build
  time RUST_LOG=info $bin solve-basic ${1:-1}
}

# * Solution / Score

best_score_refresh() {
  build
  $bin best-score-refresh
}

best_score_total() {
  cat ./solution/best-score.json | jq '[.[]] | add'
}

# score_cal_compare() {
#   for i in {1..90}; do
#     local sol=./solution/best/$i.json
#     echo "$i: $($bin score $i $sol) $($bin score2 $i $sol) "
#   done
# }

score() {
  build
  $bin score $1 $2
}

# * Submission

submit() {
  local id=$1
  local file=$2
  local token=$(api_token)
  local jq_args=(
    -n
    --argjson problem_id $id
    --arg contents $(cat $file)
    '{ "problem_id": $problem_id, "contents": $contents}'
  )
  jq ${jq_args} | curl $curl_options --header "Authorization: Bearer ${token}" --json @- $site/submission
}

submit_best() {
  local token=$(api_token)
  for i in {1..$max_problem_id}; do
    if [[ -f ./solution/best/${i}.json ]]; then
      if [[ -f ./solution/submission/${i}.json ]] && cmp --silent ./solution/best/${i}.json ./solution/submission/${i}.json ; then
        echo "Skipping ./solution/best/${i}.json"
      else
        echo "Submitting ./solution/best/${i}.json..."
        submit $i ./solution/best/${i}.json
        cp -a ./solution/best/${i}.json ./solution/submission/
        sleep 1
      fi
    fi
  done

  userboard
}

submissions() {
  local token=$(api_token)
  curl --header "Authorization: Bearer ${token}" "$site/submissions?offset=0&limit=10" \
    | jq .
}

userboard() {
  # api.icfpcontest.com/userboard
  local token=$(api_token)
  curl $curl_options --header "Authorization: Bearer ${token}" $site/userboard \
    | tee ./solution/userboard.json \
    | jq .
}

# * Watch progress

watch() {
  watchman-make -p 'render/wip.svg' --run "my-browser reload wip.svg"
}

watch_100() {
  watchman-make -p 'render/wip.svg' --run "my-browser --port 10028 reload wip.svg"
}

browser() {
  my-google-chrome --port 10028
}

movie() {
  cd ./render/wip
  ffmpeg -y -framerate 2 -pattern_type glob -i '*.svg' -codec:v vp9 -lossless 1 out.webm
}

# * Benchmark / Profiling

bench() {
  build
  hyperfine "$bin bench ${1:-60}"
}

bench_scoring() {
  build
  $bin bench-scoring ${1:-1}
}

profiling() {
  build
  LD_PRELOAD=/usr/lib/x86_64-linux-gnu/libprofiler.so CPUPROFILE=gperf-cpu.prof $bin bench ${1:-60}
}

profiling_web() {
  pprof -http=:8085 $bin ./gperf-cpu.prof
}
