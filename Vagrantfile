# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.define "centos" do |box|
    box.vm.box = "puppetlabs/centos-7.0-64-puppet"

    # config.vm.box_check_update = false
    # config.vm.network "private_network", ip: "192.168.33.10"
    box.vm.synced_folder ".", "/vagrant"
    box.ssh.forward_agent = true

    # Enable provisioning with a shell script. Additional provisioners such as
    # Puppet, Chef, Ansible, Salt, and Docker are also available. Please see the
    # documentation for more information about their specific syntax and use.
    box.vm.provision "shell", inline: <<-SHELL
    yum install -y git
    yum install -y vim
    SHELL
  end

  config.vm.define "ubuntu" do |box|
    box.vm.box = "puppetlabs/ubuntu-14.04-64-nocm"

    # config.vm.box_check_update = false
    # config.vm.network "private_network", ip: "192.168.33.10"
    box.vm.synced_folder ".", "/vagrant"
    box.ssh.forward_agent = true

    # Enable provisioning with a shell script. Additional provisioners such as
    # Puppet, Chef, Ansible, Salt, and Docker are also available. Please see the
    # documentation for more information about their specific syntax and use.
    box.vm.provision "shell", inline: <<-SHELL
    apt-get install -y git
    apt-get install -y vim
    SHELL
  end
end
