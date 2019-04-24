FROM codercom/code-server


USER root
ADD startup_coder.sh /home/coder/project
ADD install_docker.sh /home/coder/project
RUN chmod 777 /home/coder/project/startup_coder.sh
RUN chmod 777 /home/coder/project/install_docker.sh

WORKDIR /home/coder/project
RUN /bin/bash ./install_docker.sh
#RUN sudo apt-get install -y apt-utils docker
ENTRYPOINT ["/bin/bash", "startup_coder.sh" ]