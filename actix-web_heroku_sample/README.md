# setup

heroku create --buildpack emk/rust actix-web-heroku-sample

echo "web: ./target/release/actix-web_heroku_sample" > Procfile

git push heroku master
