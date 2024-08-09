Let's assume you are adding `go` as a new language.

You would create a new directory `go` under `new-lang` and add the following files:
* `LightSm.go` (or equivalent) - the manually translated version of `LightSm.c` (or other language) to `go`
* `LightSmTest.go` (or equivalent) - a basic test harness equivalent to [../c/test-harness.c](../c/test-harness.c)
* `README.md` - a file that explains how to run the tests and any other useful information
