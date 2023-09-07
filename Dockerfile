FROM archlinux:base
RUN pacman -Syu --noconfirm
RUN pacman -S jdk-openjdk gcc redland python python-matplotlib python-numpy python-pandas python-tabulate make --noconfirm

COPY . .
RUN make

ENV NODE_OPTIONS=--max_old_space_size=16000
CMD run_benchmark query hdt_cpp
