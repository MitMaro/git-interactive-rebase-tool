## Deploy Docker Images


From the project root:

```
docker login --username=<usernmae>
docker build -t "mitmaro/git-rebase-tool-circleci:latest" --file ".circleci/images/rust/Dockerfile" .

# test  the build with
docker run -v "$PWD:/app" mitmaro/git-rebase-tool-circleci:latest ./scripts/build-deb.bash

docker push mitmaro/git-rebase-tool-circleci:latest
```