# Animall

## How to run server

I have the images tagged to my local registry so run a local registry as shown below. In case you don't want to run your own registry do change the image name to a non registry tag.

`docker run -d -p 5000:5000 --name registry registry:2`

## To run the server

```
docker-compose build
docker-compose push
docker-compose up
```

## First Time setup

After first `docker-compose up` and run `cargo install diesel_cli --no-default-features --features postgres` and `diesel migration run`
