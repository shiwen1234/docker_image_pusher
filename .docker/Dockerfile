# 使用 Ubuntu 22.04 作为基础镜像
FROM ubuntu:22.04

# 设置环境变量，防止在安装过程中交互式提示
ENV DEBIAN_FRONTEND=noninteractive

# 更新并安装必要的软件包
RUN apt-get update && apt-get install -y \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg \
    lsb-release \
    software-properties-common

# 添加 Docker 官方 GPG 密钥
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo
# 设置 Docker 仓库
RUN echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
    $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# 更新 APT 包索引并安装 Docker 引擎
RUN apt-get update && apt-get install -y \
    docker-ce \
    docker-ce-cli \
    containerd.io

# 安装 Docker Compose
RUN curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose \
    && chmod +x /usr/local/bin/docker-compose

# 验证 Docker 和 Docker Compose 安装
RUN docker --version && docker-compose --version

# 设置工作目录
WORKDIR /root

# 设置默认命令
CMD ["bash"]
