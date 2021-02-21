build_moon_assistant(){
    cd $HOME/opt/moon-assistant
    mkdir -p $HOME/opt/moon-assistant/build
    cargo build
    cp $HOME/opt/moon-assistant/target/debug/moon $HOME/opt/moon-assistant/build/moon
}

build_moon(){
    cd $HOME/opt/Moon
    mkdir -p $HOME/opt/Moon/build/cmake
    next build
}

main(){
    build_moon_assistant
    build_moon
}

main