# GitHub Action: GPT Language Check

This GitHub action reviews markdown files in your pull requests for potentially discriminatory language, including but not limited to ableism, ageism, racism, antisemitism, Islamophobia, and similar issues. It uses the OpenAI GPT-4 language model to detect discriminatory language and provides suggestions for alternative language.

## Usage

To use this action in your repository, follow these steps:

1. Set up the `OPENAI_API_KEY` secret in your GitHub repository. You can find more information about setting up secrets in the [GitHub documentation](https://docs.github.com/en/actions/security-guides/encrypted-secrets).

2. Create a new file named `review_markdown.yml` in the `.github/workflows` directory of your repository.

3. Copy the contents of the [sample `review_markdown.yml` file](docs/review_markdown.yml) into the newly created file.

4. Commit and push your changes to the repository.

When a pull request is created or updated with changes to markdown files, this action will review them for potentially discriminatory language and add a comment to the pull request with suggestions for alternative language if any issues are found.

## Contributing

If you would like to contribute to this project, please read the [contributing guidelines](CONTRIBUTING.md) and [code of conduct](CODE_OF_CONDUCT.md).

## License

This project is licensed under the [MIT License](LICENSE).