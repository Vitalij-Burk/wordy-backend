# Wordy - the utility for words translating and storing them in storage

### Wordy supports multiple languages like English, Chinese, Spanish, French, German, Japanese, Russian, Arabic, and many others, relying on Google Translate API

## Main Functions:

1. Text translate
2. Collecting your saved translations in storage

## Routing:

- POST /user/create
- POST /translate
- POST /user/user_id/{user_id}/wordpair/create
- POST /user/key/{key}/wordpair/create
- GET /user/user_id/{user_id}/wordpair
- GET /user/key/{key}/wordpair

## Code Architecture:

- Clean Architecture
- Without overengineering
- With a reserve for the future

## Tech Stack:

- **Backend**: Rust + Axum + SQLx
- **Database**: PostgreSQL
- **API**: Google Translate API via translators
- **Telegram-bot**: Rust + Teloxide
- **Docker**

## TODO:

- [ ] Authorization && Authentication
- [ ] Host Backend
- [ ] Develop telegram-bot for MVP on Teloxide
- [ ] Develop CLI for MVP on Clap
- [ ] Develop Mobile App on Kotlin | Java
- [ ] Collections logics
- [ ] Use another tool for translate
- [ ] Do sorting
- [ ] Train own AI model

## If you wanna work on a Wordy with me, Project is needed in:

1. Mobile developers (Java, Kotlin)
2. Designers
3. Backend developers (Rust, Python, Go)
