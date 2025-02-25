sudo cp target/release/gitlicense /usr/bin
echo "Gitlicense binary was succesfully installed"
read -p "Would you like to add gl alias for gitlicense?(y/n)" input
if [ "$input" = "y" ];then
    if [ "$SHELL" = "/usr/bin/bash" ]; then
        echo ' alias gl = "gitlicense" ' >> $HOME/.bashrc
        source $HOME/.bashrc
        echo "Alias was succesfully added to your bash config"
    elif [ "$SHELL" = "/usr/bin/zsh" ]; then
        echo ' alias gl = "gitlicense" ' >> $HOME/.zshrc 
        source $HOME/.zshrc
        echo "Alias was succesfully added to your zsh config"
    elif [ "$SHELL" = "/usr/bin/sh" ];then
        echo ' alias gl = "gitlicense" ' >> $HOME/.profile 
        source $HOME/.profile
        echo "Alias was succesfully added to your sh profile,
        but consider moving to a more modern shell"
    # Add support for nushell
    else 
        echo "Your shell is not supported"
    fi
elif [ "$input" = "n" ];then
    echo "Ok"
else
    echo "This option is not supported"
fi
