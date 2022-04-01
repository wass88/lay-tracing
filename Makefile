movie.mp4: $(wildcard movie/*.png)
	cargo run --release
	ffmpeg -y -r 10 -i movie/movie%04d.png -c:v libx264 -vf fps=10 -pix_fmt yuv420p out.mp4


.PHONY: movie.mp4