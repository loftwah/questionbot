# QuestionBot

QuestionBot is a Rust program that sends a random question to a Discord channel using a webhook. The program reads a list of questions from a JSON file, shuffles the questions, and selects a random question to send to the channel. The program is designed to be used as a Discord bot and can be scheduled to run on a regular basis or triggered manually using GitHub Actions.

![QuestionBot](https://user-images.githubusercontent.com/19922556/218981108-28805416-6635-466b-aa4d-8baeafe7e7eb.jpg)

## Prerequisites

To use QuestionBot, you need to provide the program with two environment variables: `QUESTIONS` and `WEBHOOK`. The `QUESTIONS` environment variable should contain the path to a JSON file that contains a list of questions in the following format:

```json
[
    {
        "question": "What is your favorite color?"
    },
    {
        "question": "What is the capital of France?"
    }
]
```

The `WEBHOOK` environment variable should contain the URL of the Discord webhook that QuestionBot should use to send the questions.

To set these environment variables as secrets in your GitHub repository:

1. Go to your repository on GitHub.
2. Click on the "Settings" tab.
3. Click on the "Secrets" link in the left sidebar.
4. Click on the "New secret" button.
5. For the name, enter `QUESTIONS`.
6. For the value, enter the path to your questions JSON file. For example, if your questions file is located at `./questions.json`, you would enter `./questions.json` as the value.
7. Click on the "Add secret" button.
8. Repeat steps 4-7 for the `WEBHOOK` environment variable.

## Usage

To run QuestionBot, navigate to the project directory and run the following command:

`cargo run --release`

QuestionBot will select a random question from the list of questions and send it to the Discord channel using the webhook. The program can be scheduled to run on a regular basis using GitHub Actions or triggered manually from the Actions UI.

## GitHub Actions

To run QuestionBot on a schedule or trigger it manually using GitHub Actions, you can use the following workflow file:

```yml
name: Run QuestionBot
on:
  schedule:
    - cron: '0 9 * * *'
  workflow_dispatch:
jobs:
  run:
    runs-on: ubuntu-latest
    steps:
    - name: Checkout code
      uses: actions/checkout@v2
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: stable
    - name: Build and run
      run: |
        cargo run --release
      env:
        QUESTIONS: ${{ secrets.QUESTIONS }}
        WEBHOOK: ${{ secrets.WEBHOOK }}
```

This workflow runs QuestionBot on a schedule using a `cron` expression and allows it to be triggered manually from the Actions UI. The workflow sets the `QUESTIONS` and `WEBHOOK` environment variables using the GitHub secrets that you defined earlier.

To trigger the workflow manually, go to the Actions tab in your repository, click on the "Run workflow" button, select the "Run QuestionBot" workflow, and click the "Run workflow" button. You can also provide an optional input parameter to the workflow by clicking on the "This workflow has parameters" link and filling in the input field.

## License

QuestionBot is licensed under the MIT license. See the [LICENSE](LICENSE) file for details.
