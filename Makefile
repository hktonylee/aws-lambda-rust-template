run-cli:
	cargo run

build-lambda-arm64:
	cargo build --release --target aarch64-unknown-linux-musl

build-lambda-x86:
	cargo build --release --target x86_64-unknown-linux-musl

package-arm64:
	zip -j target/aws-lambda-artifact.zip ./target/aarch64-unknown-linux-musl/release/bootstrap

package-x86:
	zip -j target/aws-lambda-artifact.zip ./target/x86_64-unknown-linux-musl/release/bootstrap

npm-install:
	cd infrastructure; ! [ -e node_modules ] && npm install || true

# Deployment uses only ARM right now. If you need to use x86, you can change the below targets.
# And don't forget to change the CDK to use the x86 Lambda
release: build-lambda-arm64 package-arm64

deploy: npm-install release
	cd infrastructure; cdk deploy

.PHONY: build-lambda build-cli release deploy
