## 2019-04-24, Version 1.0.0
### Commits
- [[`a7ff8ea564`](https://github.com/datrs/random-access-memory/commit/a7ff8ea564f11673b3e39d605befe9b25fc58574)] (cargo-release) version 1.0.0 (Yoshua Wuyts)
- [[`76c0ebfe1b`](https://github.com/datrs/random-access-memory/commit/76c0ebfe1ba046871430f03fe5876a4e27554fb6)] Update random-access-storage to 2.0.0 (#12) (Jack Jennings)
- [[`defaf55071`](https://github.com/datrs/random-access-memory/commit/defaf55071de68206eada89a74616d26603e2faa)] Update quickcheck requirement from 0.7.1 to 0.8.1 (dependabot[bot])
- [[`42023a8700`](https://github.com/datrs/random-access-memory/commit/42023a870050fdbc196c6f7d2ffd3e7c36797fac)] Update rand requirement from 0.5.5 to 0.6.0 (dependabot[bot])
- [[`9202fe0308`](https://github.com/datrs/random-access-memory/commit/9202fe0308151dce780c7721f533bed2e11f7f3c)] Update quickcheck requirement from 0.6.2 to 0.7.1 (#6) (dependabot[bot])
- [[`eae681682e`](https://github.com/datrs/random-access-memory/commit/eae681682eef6a4fe888f25b8728951873b6f114)] Run clippy on travis (#5) (Szabolcs Berecz)
- [[`df0755085c`](https://github.com/datrs/random-access-memory/commit/df0755085c1e472b084f9598aaa2ac0d4fb04f10)] update changelog (Yoshua Wuyts)

### Stats
```diff
 .travis.yml    |  4 +++-
 CHANGELOG.md   | 20 ++++++++++++++++++++
 Cargo.toml     |  7 ++++---
 src/lib.rs     | 12 ++++++++++++
 tests/model.rs |  2 ++
 tests/test.rs  | 18 ++++++++++++++++++
 6 files changed, 59 insertions(+), 4 deletions(-)
```


## 2018-08-30, Version 0.5.0
### Commits
- [[`3baa7c2d23`](https://github.com/datrs/random-access-memory/commits/3baa7c2d23dfa774ac5e1d2b38bbb171eaf95bc0)] (cargo-release) version 0.5.0 (Yoshua Wuyts)
- [[`300ead96ab`](https://github.com/datrs/random-access-memory/commits/300ead96ab4eab5b66f786f3b0562ddb29571d27)] Random access always open (#4) (Szabolcs Berecz)
- [[`db908834f8`](https://github.com/datrs/random-access-memory/commits/db908834f8e79dd8a8f98a22e1403da3ebf458da)] update changelog (Yoshua Wuyts)

### Stats
```diff
 .travis.yml         |  1 +-
 CHANGELOG.md        | 23 +++++++++++++++++-
 Cargo.toml          |  4 +--
 benches/sync.rs     |  2 +-
 src/lib.rs          | 74 ++++++++++++++++++++++++------------------------------
 tests/model.rs      |  2 +-
 tests/regression.rs |  8 ++----
 tests/test.rs       |  4 ++-
 8 files changed, 69 insertions(+), 49 deletions(-)
```


## 2018-08-23, Version 0.4.0
### Commits
- [[`7876a1a1ca`](https://github.com/datrs/random-access-memory/commits/7876a1a1ca10913a6126b767f4d07c3bae99dd8e)] (cargo-release) version 0.4.0 (Yoshua Wuyts)
- [[`cc83784775`](https://github.com/datrs/random-access-memory/commits/cc83784775a7cc737c97d514e88cdcb9ca2e718a)] upgrade random-access-storage (#3)

* upgrade random-access-storage

* cargo fmt (Yoshua Wuyts)
- [[`fae1a6509b`](https://github.com/datrs/random-access-memory/commits/fae1a6509b3c3825b3c063a627f64f85ab11cf40)] fix rustfmt in travis.yml (Yoshua Wuyts)
- [[`51c944434b`](https://github.com/datrs/random-access-memory/commits/51c944434b46c16ef7ba4028ac31ec10d6d64fe3)] fix benches (Yoshua Wuyts)
- [[`abfd505e04`](https://github.com/datrs/random-access-memory/commits/abfd505e049da526af068f475c1e8730e33238b9)] (cargo-release) start next development iteration 0.3.1-alpha.0 (Yoshua Wuyts)

### Stats
```diff
 .travis.yml         |  2 +-
 Cargo.toml          |  4 ++--
 benches/sync.rs     |  6 +++---
 src/lib.rs          | 23 ++++++++++++++++-------
 tests/regression.rs |  4 +---
 5 files changed, 23 insertions(+), 16 deletions(-)
```


