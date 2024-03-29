#include <linux/if.h>
#include <linux/if_tun.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/fcntl.h>
#include <sys/ioctl.h>
#include <unistd.h>

int tun_alloc(char *dev, int flags)
{
    struct ifreq ifr;
    int fd, err;

    char *clonedev = "/dev/net/tun";

    if ((fd = open(clonedev, O_RDWR)) < 0) {
        return fd;
    }

    memset(&ifr, 0, sizeof(ifr));
    ifr.ifr_flags = flags;

    if (*dev != '\0') {
        strncpy(ifr.ifr_name, dev, IFNAMSIZ);
    }
    if ((err = ioctl(fd, TUNSETIFF, (void *)&ifr)) < 0) {
        close(fd);
        return err;
    }

    // 一旦设备开启成功，系统会给设备分配一个名称，对于tun设备，一般为tunX，X为从0开始的编号；
    // 对于tap设备，一般为tapX
    strcpy(dev, ifr.ifr_name);

    return fd;
}

int main(int argc, char **argv)
{
    int tun_fd, nread;
    char buffer[4096];
    char tun_name[IFNAMSIZ];

    tun_name[0] = '\0';

    /* Flags: IFF_TUN   - TUN device (no Ethernet headers)
     *        IFF_TAP   - TAP device
     *        IFF_NO_PI - Do not provide packet information
     */
    tun_fd = tun_alloc(tun_name, IFF_TUN | IFF_NO_PI);

    if (tun_fd < 0) {
        perror("Allocating interface");
        exit(1);
    }
    printf("Open tun/tap device: %s for reading...\n", tun_name);

    while (1) {
        unsigned char ip[4];
        // 收包
        nread = read(tun_fd, buffer, sizeof(buffer));
        if (nread < 0) {
            perror("Reading from interface");
            close(tun_fd);
            exit(1);
        }

        // int i;
        // for (i = 0; i < nread; i++) {
        //     printf(" %x", buffer[i]);
        // }
        // printf("\n");

        printf("Read %d bytes from tun/tap device\n", nread);

        // 简单对收到的包调换一下顺序
        memcpy(ip, &buffer[12], 4);
        memcpy(&buffer[12], &buffer[16], 4);
        memcpy(&buffer[16], ip, 4);

        buffer[20] = 0; // icmp echo

        *((unsigned short *)&buffer[22]) += 8; // checksum

        // 发包
        nread = write(tun_fd, buffer, nread);

        printf("Write %d bytes to tun/tap device, that's %s\n", nread, buffer);
    }
    return 0;
}