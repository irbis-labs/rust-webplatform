test:
    java -version || (echo "You should install JAVA" && false)
    geckodriver --version || (echo "You should install geckodriver" && false)
    mkdir -p .test-runner
    echo 'e270cb3075cde0f35d176a56403990e5361077e6  .test-runner/test-runner.jar' | sha1sum -c \
      || wget -O .test-runner/test-runner.jar https://github.com/vlad20012/rust-wasm-test-runner/releases/download/0.1/rust-wasm-test-runner-all-0.1.jar
    cargo test --no-run --target=wasm32-unknown-emscripten
    cargo test --no-run --target=wasm32-unknown-emscripten --message-format=json | java -jar .test-runner/test-runner.jar --input

