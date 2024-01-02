# Release Checklist

- Ensure local `master` is up to date with respect to `origin/master`.
- Run `cargo update` and review dependency updates. Commit updated `Cargo.lock`.
- Edit the `Cargo.toml` to set the new count-hours version.
- Run `cargo update -p count-hours` so that the `Cargo.lock` is updated.
  Commit and push the changes.
- Check that the CI is passing.
- Create and push the new tag with: `TAG_NAME=<tag_name>; git tag -s $TAG_NAME -m $TAG_NAME && git tag -v $TAG_NAME && git push origin $TAG_NAME`
- Wait for CI to finish creating the release.
  If the release build fails, then delete the tag from GitHub, make fixes, re-tag, delete the release and push.
