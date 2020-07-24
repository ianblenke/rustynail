all:
	diesel || cargo install diesel_cli
	diesel migration run
