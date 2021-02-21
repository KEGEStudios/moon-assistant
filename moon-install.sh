check_install_cargo()
{
    CMAKE_VERSION="cargo --version"
    RETVAL=0
    if eval $CMAKE_VERSION 
    then
        RETVAL=1
        printf "\n"
    else
        echo "Not install Cargo or Rust"
    fi
    return $RETVAL
}

check_install_git()
{
    GIT_VERSION="git --version"
    RETVAL=0
    if eval $GIT_VERSION 
    then
        RETVAL=1
        printf "n"
    else
        echo "Not install git"
    fi
    return $RETVAL
}

install_next(){
    curl -s https://raw.githubusercontent.com/KEGEStudios/Next/master/next-install.sh | bash -s
}

clone_moon_assistant_rep(){
    mkdir -p $HOME/opt
    cd $HOME/opt
    git clone https://github.com/KEGEStudios/moon-assistant
}

build_moon-assistant(){
    cd $HOME/opt/Next
    mkdir -p $HOME/opt/moon-assistant/build
    cargo build
    cp $HOME/opt/moon-assistant/target/debug/moon $HOME/opt/moon-assistant/build/moon
}

clone_moon_rep(){
    mkdir -p $HOME/opt
    cd $HOME/opt
    git clone https://github.com/KEGEStudios/Moon
}

main(){
    check_install_git
    check_install_cargo
    install_next
    clone_moon_assistant_rep
    build_moon-assistant
    clone_moon_rep
}

main