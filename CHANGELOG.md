## 2023-04-12, Version 3.0.0
### Commits
- [[`ef600832dd`](https://github.com/datrs/random-access-memory/commit/ef600832ddc13d2ba90250ef710a9e0b142e4bad)] Release 3.0.0 (Timo Tiuraniemi)
- [[`e6d7e97ed1`](https://github.com/datrs/random-access-memory/commit/e6d7e97ed1ca4e953a8279e79b614db15562b999)] Switch from Travis to GHA (Timo Tiuraniemi)
- [[`08a7da6cdd`](https://github.com/datrs/random-access-memory/commit/08a7da6cdd016c4fc8961849de9ec716a18e2a19)] Use released 5.0.0 of random-access-storage (Timo Tiuraniemi)
- [[`d69cde848c`](https://github.com/datrs/random-access-memory/commit/d69cde848c3d9591c818739065c122a019f862f4)] Switch from quickcheck to proptest (Timo Tiuraniemi)
- [[`27f5c02748`](https://github.com/datrs/random-access-memory/commit/27f5c0274806b793bb3fa1101bd948ee3046db35)] Use beta.0 as the version until dependencies are bumped (Timo Tiuraniemi)
- [[`2c74bccf2c`](https://github.com/datrs/random-access-memory/commit/2c74bccf2c4922ac7507616e66e0dce5af3d0bd8)] Set version to 3.0.0, bump edition to 2021 (Timo Tiuraniemi)
- [[`c47c2db567`](https://github.com/datrs/random-access-memory/commit/c47c2db5674a327d4397b72d24f13c3fbef273f9)] Fix clippy issues, forbid unsafe code (Timo Tiuraniemi)
- [[`fcd398ada1`](https://github.com/datrs/random-access-memory/commit/fcd398ada15ed9f50a0483884a98cfd94b9ae700)] Complete documentation (Timo Tiuraniemi)
- [[`5dd26f3165`](https://github.com/datrs/random-access-memory/commit/5dd26f316551dd343dff81ffed9b1e64229a4162)] Use a single OutOfBounds error for more convenient error matching (Timo Tiuraniemi)
- [[`9ebea000d6`](https://github.com/datrs/random-access-memory/commit/9ebea000d63f63ad5b0532ef2cb37c870ea0d754)] Switch from anyhow to thiserror and RandomAccessError (Timo Tiuraniemi)
- [[`ecf536f285`](https://github.com/datrs/random-access-memory/commit/ecf536f2852d436f018f286a40a9fedd8774a3af)] Enforce delete offset and truncate when larger (Timo Tiuraniemi)
- [[`606b1e33eb`](https://github.com/datrs/random-access-memory/commit/606b1e33eb291b5d87673fa8414b916ef43fa890)] Migrate bench from libtest to criterion, add del test (Timo Tiuraniemi)
- [[`b73a3c653e`](https://github.com/datrs/random-access-memory/commit/b73a3c653e99a1b3b97a28f87446f35d57ce5816)] Use tokio-compatible RandomAccessStorage (Timo Tiuraniemi)
- [[`3e90f7cc41`](https://github.com/datrs/random-access-memory/commit/3e90f7cc41ef565bd496678dca515041f8a6ed9e)] Fix len() missing &mut (Timo Tiuraniemi)
- [[`5b064d7e71`](https://github.com/datrs/random-access-memory/commit/5b064d7e7174d344592e0da2e1d2a1dce4960865)] feat: del and truncate using intmap (Timo Tiuraniemi)
- [[`6c9761f53a`](https://github.com/datrs/random-access-memory/commit/6c9761f53a3da5fed7b0400cf949958bb3fe5f24)] Update changelog (Bruno Tavares)

### Stats
```diff
  .github/workflows/ci.yml |  95 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 CHANGELOG.md             |  20 +++++++++++++++
 Cargo.toml               |  26 +++++++++++--------
 README.md                |  25 +++++-------------
 benches/sync.rs          |  59 +++++++++++++++++++++++++++---------------
 src/lib.rs               | 287 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++-----------------------------------------------------------
 tests/model.rs           |  77 +++++++++++++++++++++++++++++++++++++------------------
 tests/test.rs            |  96 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 8 files changed, 526 insertions(+), 159 deletions(-)
```


## 2020-03-03, Version 2.0.0
### Commits
- [[`cab67711c4`](https://github.com/datrs/random-access-memory/commit/cab67711c42f6229fcf01fac0bfae0b5d0a3fe86)] (cargo-release) version 2.0.0 (Bruno Tavares)
- [[`fc07e9cfef`](https://github.com/datrs/random-access-memory/commit/fc07e9cfefdc808f950fada80df7bfe697eaad6d)] Implement async API for random-access-storage (#26) (Bruno Tavares)
- [[`c10203a920`](https://github.com/datrs/random-access-memory/commit/c10203a9206c6e5f93f31b34998096fa1828d947)] Update changelog (Bruno Tavares)

### Stats
```diff
 CHANGELOG.md        | 13 +++++++++++++
 Cargo.toml          |  7 +++++--
 README.md           |  8 ++++----
 benches/sync.rs     | 39 +++++++++++++++++++++++----------------
 src/lib.rs          | 28 ++++++++++++++++++----------
 tests/model.rs      | 52 +++++++++++++++++++++++++++-------------------------
 tests/regression.rs | 36 ++++++++++++++++++------------------
 tests/test.rs       | 52 ++++++++++++++++++++++++++--------------------------
 8 files changed, 134 insertions(+), 101 deletions(-)
```


## 2020-03-03, Version 1.2.0
### Commits
- [[`d9fbf73cf1`](https://github.com/datrs/random-access-memory/commit/d9fbf73cf182e6f02e5adc4c016afe38e8650a88)] (cargo-release) version 1.2.0 (Bruno Tavares)
- [[`0b845fbcc7`](https://github.com/datrs/random-access-memory/commit/0b845fbcc747093d8c9eae972e92c8d4a692a208)] Move from failure to use stderr (#25) (Bruno Tavares)

### Stats
```diff
 Cargo.toml |  4 ++--
 src/lib.rs | 24 +++++++++++++-----------
 2 files changed, 15 insertions(+), 13 deletions(-)
```


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


