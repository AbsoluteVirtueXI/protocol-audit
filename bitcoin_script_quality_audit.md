# Bitcoin Script module code quality audit

This is a quick code quality review of the [Bitcoin Script](https://github.com/bitcoin/bitcoin/tree/master/src/script) module.

## Compilation Errors and Warnings

Compiling the bitcoin source code triggered warnings at compilation time for the script module:

```text
script/descriptor.cpp:1557:21: warning: loop variable 'keyspan' of type 'const Span<const unsigned char>' creates a copy from type 'const Span<const unsigned char>' [-Wrange-loop-analysis]
    for (const auto keyspan : match->second) {
                    ^
script/descriptor.cpp:1557:10: note: use reference type 'const Span<const unsigned char> &' to prevent copying
    for (const auto keyspan : match->second) {
         ^~~~~~~~~~~~~~~~~~~~
                    &
1 warning generated.
```

## Cppcheck static code analysis

`cppcheck` ran without a single warning or error:

```zsh
$ cppcheck src/script --force
Checking src/script/bitcoinconsensus.cpp ...
1/10 files checked 1% done
Checking src/script/descriptor.cpp ...
2/10 files checked 30% done
Checking src/script/interpreter.cpp ...
3/10 files checked 61% done
Checking src/script/miniscript.cpp ...
4/10 files checked 68% done
Checking src/script/script.cpp ...
5/10 files checked 72% done
Checking src/script/script_error.cpp ...
6/10 files checked 75% done
Checking src/script/sigcache.cpp ...
7/10 files checked 76% done
Checking src/script/sign.cpp ...
8/10 files checked 87% done
Checking src/script/signingprovider.cpp ...
9/10 files checked 90% done
Checking src/script/standard.cpp ...
10/10 files checked 100% done
```

## clang-tidy static code analysis

`clang-tidy` ran with a lot of warnings and errors. Almost all of them are false positives.  
I had hard time making clang-tidy running correctly on my machine on specific part of the Bitcoin code base.  
clang-tidy extension for Clion IDE gave me less false positives.  
An ugly trick i have used is piping the output of clang-tidy to a grep command and match only source code containing path "src/script". I have found this way 275 errors.

```zsh
/usr/local/bin/clang-tidy src/script/*.cpp -extra-arg=-ferror-limit=0 -- -Isrc/ | grep -C 4 "src/script" > error.txt
```

## Unit tests

Script module is fully covered by 48 unit tests.  
We can launch each unit tests related to Script module with the `test_bitcoin` binary generated at compilation of Bitcoin.

```zsh
$ src/test/test_bitcoin  --run_test=script_p2sh_tests
Running 6 test cases...

*** No errors detected
$ src/test/test_bitcoin  --run_test=script_parse_tests
Running 1 test case...

*** No errors detected
$ src/test/test_bitcoin  --run_test=script_segwit_tests
Running 12 test cases...

*** No errors detected
$ src/test/test_bitcoin  --run_test=script_standard_tests
Running 7 test cases...

*** No errors detected
$ src/test/test_bitcoin  --run_test=script_tests
Running 20 test cases...

*** No errors detected
$ src/test/test_bitcoin  --run_test=scriptnum_tests
Running 2 test cases...

*** No errors detected
```

## bitcoin official coding style

https://github.com/bitcoin/bitcoin/blob/master/doc/developer-notes.md
