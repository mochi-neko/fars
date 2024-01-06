#!/usr/bin/bash
# This script is used to run the examples in the examples directory.

# Set credentials for each sign in method.

# export FIREBASE_API_KEY=""
# export FIREEBASE_PROJECT_ID=""

EMAIL="t.o.e.4315@gmail.com"
PASSWORD="password"

DUMMY_EMAIL="t.o.e.4315+1@gmail.com"
DUMMY_PASSWORD="password"

REFRESH_TOKEN=""

REQUEST_URI="http://localhost"
GOOGLE_ID_TOKEN=""
FACEBOOK_ACCESS_TOKEN=""
TWITTER_ACCESS_TOKEN=""
TWITTER_OAUTH_TOKEN_SECRET=""

DISPLAY_NAME="Mochineko"
PHOTO_URL="https://avatars3.githubusercontent.com/u/12690315?s=460&v=4"

# Run examples for siginig in methods.

cargo run --example sign_up_with_email_password -- --email $DUMMY_EMAIL --password $DUMMY_PASSWORD
cargo run --example delete_account -- --email $DUMMY_EMAIL --password $DUMMY_PASSWORD
cargo run --example sign_in_with_email_password -- --email $EMAIL --password $PASSWORD
cargo run --example sign_in_anonymously
if [ ${REFRESH_TOKEN} -ne "" ]; then
    cargo run --example sign_in_by_refresh_token -- --refresh_token $REFRESH_TOKEN
fi
if [ ${GOOGLE_ID_TOKEN} -ne "" ]; then
    cargo run --example sign_in_google_oauth_credential -- --request_uri $REQUEST_URI --id_token $GOOGLE_ID_TOKEN
fi
if [ ${FACEBOOK_ACCESS_TOKEN} -ne "" ]; then
    cargo run --example sign_in_with_facebook_oauth_credential -- --request_uri $REQUEST_URI --access_token $FACEBOOK_ACCESS_TOKEN
fi
if [ ${TWITTER_ACCESS_TOKEN} -ne "" ]; then
    cargo run --example sign_in_twitter_oauth_credential -- --request_uri $REQUEST_URI --access_token $TWITTER_ACCESS_TOKEN --oauth_token_secret $TWITTER_OAUTH_TOKEN_SECRET
fi

# Run examples for a not signing in user.

cargo run --example send_password_reset_email -- --email $EMAIL
cargo run --example fetch_providers_for_email -- --email $EMAIL

# Run examples for a signing in user.

cargo run --example get_user_data -- --email $EMAIL --password $PASSWORD
cargo run --example update_profile -- --email $EMAIL --password $PASSWORD --display_name $DISPLAY_NAME --photo_url $PHOTO_URL
cargo run --example delete_profile -- --email $EMAIL --password $PASSWORD
cargo run --example send_email_verification -- --email $EMAIL --password $PASSWORD
cargo run --example refresh_token -- --email $EMAIL --password $PASSWORD

# Run examples for a signing in user with a credential.

cargo run --example sign_up_with_email_password -- --email $DUMMY_EMAIL --password $DUMMY_PASSWORD
# Skip because changing email is not allowed for an unverified email.
# cargo run --example change_email -- --email $DUMMY_EMAIL --password $DUMMY_PASSWORD --new-email $DUMMY_EMAIL
cargo run --example change_password -- --email $DUMMY_EMAIL --password $DUMMY_PASSWORD --new-password $DUMMY_PASSWORD
cargo run --example delete_account -- --email $DUMMY_EMAIL --password $DUMMY_PASSWORD

cargo run --example link_with_email_password -- --email $DUMMY_EMAIL --password $DUMMY_PASSWORD
cargo run --example unlink_password -- --email $DUMMY_EMAIL --password $DUMMY_PASSWORD

if [ ${GOOGLE_ID_TOKEN} -ne "" ]; then
    cargo run --example link_with_google -- --request_uri $REQUEST_URI --id_token $GOOGLE_ID_TOKEN
    cargo run --example unlink_google -- --request_uri $REQUEST_URI --id_token $GOOGLE_ID_TOKEN
fi

# Run examples for error handling.

cargo run --example handle_error -- --email $EMAIL --password $PASSWORD

# Run examples for raw APIs.

cargo run --example raw_sign_in_with_email_password --features raw -- --email $EMAIL --password $PASSWORD

# Run examples for ID token verification.

cargo run --example verify_id_token --features verify -- --email $EMAIL --password $PASSWORD

