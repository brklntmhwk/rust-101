- Added rust-analyzer formatter to .vscode/settings.json

  - https://qiita.com/nobushi95/items/178431541c6505121b76

- Module system

  - Packages
  - Crates
    - binary
      - cargo new --bin Project-Name
    - library
      - cargo new Project-Name --lib

- Privacy

  - basically, Rust works on the premise that every element is private by default

- Test
  - run only the tests pertaining to the area in which you're focused on
    - cargo test ${test-name}
      - even if the specified test name doesn't completely match a certain test, the test for it will run as long as its name includes the keyword more or less (e.g. add for add_two and add_three)
  - useful flags
    - cargo test -- --show-output
      - tells Rust to show any println! messages encountered during tests in the outcomes
    - cargo test -- --test-threads=${n}
      - can designate how many test threads to run in parallel
      - useful when any of test units could cause data conflict or affect others' outcome
      - n is the number of threads that can run in parallel (in Rust, multiple tests run in parallel by default)
    - cargo test -- --ignored
      - runs only the ignored tests
    - cargo test -- --include-ignored
      - runs all tests whether ignored or not
    - cargo test --test integration_test
      - runs the specified integration test
