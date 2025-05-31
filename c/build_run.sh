# use a *nix shell to run this script
# if this fails to run like below, I've probably accidentally checked in \r\n newlines. Just added gitattributes. I sure dislike \r\n...
    # ./build_run.sh: line 2: $'\r': command not found
    # ./build_run.sh: line 4: cd: $'.\r': No such file or directory
    # ./build_run.sh: line 5: $'\r': command not found

# change to the directory of this script
cd "$(dirname "$0")"

gcc -g -Wall *.c -o test
./test