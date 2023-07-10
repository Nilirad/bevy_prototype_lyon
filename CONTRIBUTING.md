# Contributing to `bevy_prototype_lyon`

Thank you for your interest in contributing to `bevy_prototype_lyon`!
Before getting started,
please take a moment to review this guide
to ensure that your contributions align with our goals and expectations.

## Contribution workflow

We use a pull request strategy
to make changes to the repository.
This will help keep development organized.
Here is how you can proceed:

- Fork the repository and clone it to your local machine.
- Create a new branch for your changes.
  See the "Branches" section
  for choosing the right base branch.
- Once you've made your changes,
  push them to your fork.
- Submit a pull request to the main repository.
  Check GitHub's documentation to learn more about [pull requests].

Then,
suggestions from maintainers and other members
will lead to a refinement of the pull request.
If the changes are beneficial to the community as a whole,
the pull request will be merged.

## General contribution guidelines

Please keep the following guidelines in mind
when contributing to `bevy_prototype_lyon`:

- Before starting work on a new feature or bug fix,
  please check the [issues] page
  to see if it has already been reported.
  If not,
  feel free to start working on a pull request.
  If you think it is a potentially controversial change,
  please create a new issue
  before starting work.
- Test your code changes with `cargo test` and by running examples
  to ensure that they work as expected
  and do not break existing functionality.
- Please follow our coding conventions and standards.
  Code should be clear,
  easy to read,
  and properly documented.
- When submitting a pull request,
  please provide a clear and concise description
  of the changes you have made
  and why they are necessary.

### Branches

We maintain two long term branches,
each with their characteristics.

- `master`.
  Tracks the latest Bevy release.
  You should target this branch by default.
- `bevy-main`.
  Tracks the `main` branch of `bevy`.
  You should target this branch
  only if you have to accommodate a breaking change caused by Bevy,
  or if you want to develop a feature
  not yet supported by the latest release.

## Reporting Issues

If you encounter any issues,
please report them on the [issues] page.
Please provide as much information as possible,
including steps to reproduce the issue
and any error messages that were displayed.

## Suggestions

If you have any suggestions,
feel free to open an issue on the [issues] page.
Make sure to explain the motivation behind the desired change.
Keep in mind that some suggestions
may have side effects
that negatively impact established use cases.
Therefore they should be thoroughly discussed
before implementing them.

[issues]: https://github.com/Nilirad/bevy_prototype_lyon/issues
[pull requests]: https://docs.github.com/en/pull-requests
