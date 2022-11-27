The following was observed in the stable manifest for 2022-11-03:

```
[pkg.miri-preview]
version = ""
[pkg.miri-preview.target.aarch64-apple-darwin]
available = false
```

Both `version` (even though it's the empty string in this case) and
`git_commit_sha` have been made optional to cover the case when there was
apparently no source to build from.

Reason why components are included in the manifest but marked as `available =
false`: https://github.com/rust-lang/rust/pull/53715
