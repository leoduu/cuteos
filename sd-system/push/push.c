#include <stdio.h>     /*标准输入输出定义*/
#include <stdlib.h>    /*标准函数库定义*/
#include <unistd.h>    /*Unix标准函数定义*/
#include <sys/types.h> /**/
#include <sys/stat.h>  /**/
#include <fcntl.h>     /*文件控制定义*/
#include <termios.h>   /*PPSIX终端控制定义*/
#include <errno.h>     /*错误号定义*/
#include <signal.h>
#include <string.h>

#define TRUE  1
#define FALSE 0

typedef unsigned char u8;

#define dev_num 6
char *dev[dev_num] = {"/dev/ttyUSB0", "/dev/ttyUSB1", "/dev/ttyUSB2", "/dev/ttyUSB3", "/dev/ttyUSB4", "/dev/ttyUSB5"};
char board[] = "Firefly rk3399";
char fpath[] = "kernel.bin";

int baud = B1500000;
int delay = 30000;
int fd = 0;
int child_pid = 0;

void binary_load(int fd);
void receive_process(void);
void transmit_process(void);
int  open_dev(char *dev);

int  uart_init(int fd, int speed, int databits, int stopbits, int parity);

static void sig_handle0(int sig) 
{
    if (sig == SIGINT 
        | sig == SIGKILL
        | sig == SIGHUP) {
        if (fd) close(fd);
        printf("\n[MP] Bye 👋\n");
        exit(0);
    }
}

static void sig_handle1(int sig) 
{
    if (sig == SIGUSR1) {
        binary_load(fd);
    }    
    if (sig == SIGINT 
        | sig == SIGKILL
        | sig == SIGHUP) {
        printf("\n");
        exit(0);
    }
}

// 主函数
int main(int argc, char **argv)
{   

    if (argc>=2 && !strcmp(argv[1], "rpi4")) {
        memset(board, 0, strlen(board));
        memcpy(board, "Raspberry pi4", 14);
        baud = B921600;
        delay = 40000;
    } 

    if (argc==3) {
        memset(fpath, 0, strlen(fpath));
        memcpy(fpath, argv[2], strlen(argv[2]));
    }

    printf("%s %d %s\n",board, baud, fpath);

    printf("[MP] ⏳ Waiting for Serial\n");
    int i=0;
    while ( (fd = open_dev(dev[i])) == -1) 
        i = (i+1) % dev_num;
    printf("[MP] ✅ Serial connected %s\n", dev[i]);    

    if (uart_init(fd, baud, 8, 1, 'N') == FALSE) {
        printf("Set Parity Error\n");
        exit(1);
    }

    tcflush(fd, TCIOFLUSH);

    switch ((child_pid=fork())) {
    case -1 : 
        printf("fork Error\n");
        exit(1);
    case 0 : 
        // child process connect to Tx
        signal(SIGUSR1, sig_handle1);
        signal(SIGINT,  sig_handle1);
        transmit_process();
        break;
    default: 
        // father process connect to Rx
        signal(SIGINT, sig_handle0);
        receive_process();
        break;
    }

    while (1) {
        printf("!!!!!!!!!\n");
        usleep(1000);
    }
}


void receive_process(void) 
{
    int offset, len, num = 0;
    char ch;  
    
    unsigned char buf[2048];

    tcflush(fd, TCIOFLUSH);

    offset = 0;

    while (1) {

        if ( (len=read(fd, buf+offset, sizeof(buf)-offset)) > 0) {
            
            //printf("\n  === offset:%d len:%d ===\n", offset, len);
            for (int i=0; i<len; i++) {
                putchar(buf[offset+i]);
                // if (buf[offset+i] == '\n') printf("\n");
                // else printf("<%.2X>", buf[offset+i]);

                if (buf[offset+i] == 6) {
                    num += 1;
                    if ( num == 3) {
                        kill(child_pid, SIGUSR1);
                        num = 0;
                    }
                } else {
                    num = 0;
                }
            }

            offset += len;

            if (offset > 1024) {
                offset = 0;
                memset(buf, 0 , sizeof(buf));
            }
        }

        usleep(100);
    }
}


void transmit_process(void) 
{
    char ch;

    while (1) {
        while (read(STDIN_FILENO, &ch, 1) > 0) {
            write(fd, &ch, 1);
        }
        tcflush(fd, TCOFLUSH);             
    }
}

// 加载二进制文件
void binary_load(int fd)
{
    struct stat s;
    int size, isize, psize = 0;
    FILE *fp;
    unsigned char buf[1024];

    stat(fpath, &s);
    size = s.st_size;

    tcflush(fd, TCOFLUSH);
    
    if ((fp = fopen(fpath, "r")) < 0) {
        printf("Can't open %s\n", fpath);
        exit(-1);
    }

    printf("[MP] ⏩ Pushing %d KB\n", size);
    write(fd, (unsigned char*)(&size), 4);

    int send_size = 0;
    int rate;
    while ((isize = fread(buf, sizeof(unsigned char), 1024, fp)) > 0) {
        write(fd, buf, isize);        
        // 确保发送完毕
        send_size += isize;
        rate = send_size*100/size;

        // 进度条
        printf("\r\b[MP] ⚡\t%d%% [", rate);
        for (int i=0; i<40; i++) {
            if (i*2.5 > rate) 
                printf(" ");
            else 
                printf(">");
        }
        printf("]");

        tcdrain(fd);
        usleep(1);
    }

    printf("\r\b[MP] 🦀 finish load                                     \n");
}

/**
*@brief   设置串口数据位，停止位和校验位
*@param   fd       类型  int  打开的串口文件句柄*
*@param   databits 类型  int  数据位   取值 为 7 或者8*
*@param   stopbits 类型  int  停止位   取值为 1 或者2*
*@param   parity   类型  int  校验类型 取值为N,E,O,,S
*/

/*
struct termios
{
    unsigned short c_iflag;  //输入模式
    unsigned short c_oflag;  //输出模式
    unsigned short c_cflag;  //控制模式
    unsigned short c_lflag;  //本地模式
    unsigned char c_cc[NCC]; //控制 字符 数据
}
*/
/*
    IXON--启用输出的 XON/XOFF 流控制
    IXOFF--启用输入的 XON/XOFF 流控制
    IXANY--允许任何字符来重新开始输出
    IGNCR--忽略输入中的回车
    
    ICANON--启用标准模式 (canonical mode)。允许使用特殊字符 EOF, EOL,
            EOL2, ERASE, KILL, LNEXT, REPRINT, STATUS, 和 WERASE，以及按行的缓冲。
    ECHO--回显输入字符
    ECHOE--如果同时设置了 ICANON，字符 ERASE 擦除前一个输入字符，WERASE 擦除前一个词
    ISIG--当接受到字符 INTR, QUIT, SUSP, 或 DSUSP 时，产生相应的信号
*/
int uart_init(int fd, int speed, int databits, int stopbits, int parity)
{
    struct termios opt;
    // 获取终端参数，成功返回零；失败返回非零，发生失败接口将设置errno错误标识 返回的结果保存在termios结构体
    if (tcgetattr(fd, &opt) != 0)
    {
        perror("SetupSerial 1");
        return (FALSE);
    }
    
    tcflush(fd, TCIOFLUSH);

    // 设置波特率
    cfsetispeed(&opt, speed);
    cfsetospeed(&opt, speed);

    // 控制模式标志，指定终端硬件控制信息
    opt.c_cflag &= ~CSIZE;
    opt.c_iflag &= ~(IXON | IXOFF | IXANY);
    opt.c_oflag &= ~OPOST; //启用输出处理
    opt.c_lflag &= ~ECHO; 

    // 设置数据位数
    switch (databits) {
        case 7:
            opt.c_cflag |= CS7;
            break;
        case 8:
            opt.c_cflag |= CS8;
            break;
        default:
            // int fprintf(FILE *stream, const char *format, ...)
            fprintf(stderr, "Unsupported data size\n");
            return (FALSE);
    }
    
    // 设置校验位
    switch (parity) {
        case 'n':
        case 'N':
            opt.c_cflag &= ~PARENB; /* Clear parity enable */
            opt.c_iflag &= ~INPCK;  /* Enable parity checking */
            break;
        case 'o':
        case 'O':
            opt.c_cflag |= (PARODD | PARENB); /* 设置为奇校验*/
            opt.c_iflag |= INPCK;             /* Disnable parity checking */
            break;
        case 'e':
        case 'E':
            opt.c_cflag |= PARENB;  /* Enable parity */
            opt.c_cflag &= ~PARODD; /* 转换为偶校验*/
            opt.c_iflag |= INPCK;   /* Disnable parity checking */
            break;
        case 'S':
        case 's': /*as no parity*/
            opt.c_cflag &= ~PARENB;
            opt.c_cflag &= ~CSTOPB;
            break;
        default:
            fprintf(stderr, "Unsupported parity\n");
            return (FALSE);
    }

    // 设置停止位
    switch (stopbits) {
        case 1:
            opt.c_cflag &= ~CSTOPB;
            break;
        case 2:
            opt.c_cflag |= CSTOPB;
            break;
        default:
            fprintf(stderr, "Unsupported stop bits\n");
            return (FALSE);
    }
    // 设置输入奇偶校验选项
    if (parity != 'n')
        opt.c_iflag |= INPCK;
    // 超时时间15秒
    opt.c_cc[VTIME] = 150;
    opt.c_cc[VMIN] = 0;

    // 清空终端未完成的输入/输出请求及数据
    tcflush(fd, TCIFLUSH);
    // 设置终端参数
    if (tcsetattr(fd, TCSANOW, &opt) != 0) {

        perror("SetupSerial 3");
        return (FALSE);
    }
    return (TRUE);
}

// 打开串口
int open_dev(char *dev)
{
    int fd = open(dev, O_RDWR | O_NOCTTY | O_NDELAY | O_NONBLOCK);
    if (-1 == fd) 
        return -1;
    else
        return fd;
}
