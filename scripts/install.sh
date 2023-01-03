#!/usr/bin/env bash

{ # This ensures the entire script is downloaded.
	wget https://raw.githubusercontent.com/TiagoCavalcante/kth/main/scripts/kth.tar.gz
	tar -xvf kth.tar.gz
	chmod +x kth
	sudo cp kth /usr/local/bin/kth
	echo "kth is now installed! 🚀"
}
