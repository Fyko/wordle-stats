<div align="center">
	<img src="https://tugboat.dev/9yf7wb.png" alt="Example Grafana Panel"/>
	<h1><b>Wordle Statistics</b></h1>
	<h3>A cool program to create pretty graphs.</h3>
</div>

## How does it work?
We stream Twitter filtering tweets with the word `Wordle`, run a [parser](parser) over each one, and report the positive hits.

## Okay, cool. How do I use it?
1. Rename `.env.example` to `.env` and update the necessary fields.
1. `./y.sh dev up` to start Prometheus and Grafana.
2. Run `envup && cargo run` to start the Twitter listener.

You can access Grafana at http://127.0.0.1:3030.

### Note
At the time on initial commit, running the listener in Docker is not supported.