FROM codercom/code-server



USER root
ADD startup_coder.sh /home/coder/project
RUN chmod 777 /home/coder/project/startup_coder.sh

WORKDIR /home/coder/project
ENTRYPOINT ["/bin/bash", "startup_coder.sh" ]