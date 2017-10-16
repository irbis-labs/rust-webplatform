test_runner_url = "https://github.com/vlad20012/rust-wasm-test-runner/releases/download/0.1/rust-wasm-test-runner-all-0.1.jar"
test_runner_hash = "e270cb3075cde0f35d176a56403990e5361077e6"

test:
    java -version || (echo "You should install JAVA" && false)
    geckodriver --version || (echo "You should install geckodriver" && false)
    mkdir -p .test-runner
    echo '{{test_runner_hash}}  .test-runner/test-runner.jar' | sha1sum -c \
      || wget -O .test-runner/test-runner.jar {{test_runner_url}} \
      && echo '{{test_runner_hash}}  .test-runner/test-runner.jar' | sha1sum -c
    cargo test --no-run --target=wasm32-unknown-emscripten
    cargo test --no-run --target=wasm32-unknown-emscripten --message-format=json | java -jar .test-runner/test-runner.jar --input

