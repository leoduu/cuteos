
# SD卡系统制作

gboot 模拟uboot功能，简单初始化并将加载内核  

## Boot flow

[rk3399的wiki](http://opensource.rock-chips.com/wiki_Boot_option)上展示了引导流程

- use U-Boot TPL/SPL from upsream or rockchip U-Boot, fully source code;
- use Rockchp idbLoader which is combinded by Rockchip ddr init bin and miniloader bin from Rockchip rkbin project;

```empty
+--------+----------------+----------+-------------+---------+
| Boot   | Terminology #1 | Actual   | Rockchip    | Image   |
| stage  |                | program  |  Image      | Location|
| number |                | name     |   Name      | (sector)|
+--------+----------------+----------+-------------+---------+
| 1      |  Primary       | ROM code | BootRom     |         |
|        |  Program       |          |             |         |
|        |  Loader        |          |             |         |
|        |                |          |             |         |
| 2      |  Secondary     | U-Boot   |idbloader.img| 0x40    | pre-loader
|        |  Program       | TPL/SPL  |             |         |
|        |  Loader (SPL)  |          |             |         |
|        |                |          |             |         |
| 3      |  -             | U-Boot   | u-boot.itb  | 0x4000  | including u-boot and atf
|        |                |          | uboot.img   |         | only used with miniloader
|        |                |          |             |         |
|        |                | ATF/TEE  | trust.img   | 0x6000  | only used with miniloader
|        |                |          |             |         |
| 4      |  -             | kernel   | boot.img    | 0x8000  |
|        |                |          |             |         |
| 5      |  -             | rootfs   | rootfs.img  | 0x40000 |
+--------+----------------+----------+-------------+---------+
```

![boot](http://opensource.rock-chips.com/wiki_File:Rockchip_bootflow20181122.jpg)

这里使用rkbin获取SPL，制作一个SD卡的系统，需要以下固件

- idbloader.img
- trust.img
- uboot(这里用自己制作的曰gboot)
- boot(这里通过uart下载到开发板)

```bash
# 下载u-boot源码
git clone https://github.com/rockchip-linux/u-boot.git --depth=1

# 下载rkbin
git clone https://github.com/rockchip-linux/rkbin.git --depth=1
```

## idbloader.img

由启动ddr的固件，加上加载trust.img和uboot的miniloader组成

```bash
# 将rk3399_ddr_800MHz_v1.25.bin和rk3399_miniloader_v1.26.bin复制到u-boot目录下
cd u-boot
cp ../rkbin/bin/rk33/rk3399_ddr_800MHz_v1.25.bin ./
cp ../rkbin/bin/rk33/rk3399_miniloader_v1.26.bin ./
tools/mkimage -n rk3399 -T rksd -d rk3399_ddr_800MHz_v1.25.bin idbloader.img
cat rk3399_miniloader_v1.26.bin >> idbloader.img
```

## trust.img

rockchip提供ATF(Arm Trust Firmware)的固件，通过下面命令的到trust.img

```bash
cd rkbin
./tools/trust_merger ./RKTRUST/RK3399TRUST.ini
```

## uboot

使用项目中经过objcopy得到的二进制文件gboot.bin，生成uboot格式的文件gboot.img

```bash
cd u-boot #进入u-boot目录
tools/loaderimage --pack --uboot gboot.bin gboot.img
```

## 复制到SD卡

将生成好的.img文件通过dd命令写入SD卡的制定位置

```bash
#!/bin/bash

sd_path=/dev/sdc

sync
sudo dd if=./idbloader.img  of=$sd_path seek=64      #0x40
sudo dd if=./gboot.img      of=$sd_path seek=16384   #0x4000
sudo dd if=./trust.img      of=$sd_path seek=24576   #0x6000
sync
```
