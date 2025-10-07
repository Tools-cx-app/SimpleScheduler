#!/system/bin/sh
LOCALE=$(getprop persist.sys.locale)

local_print() {
	if [ $LOCALE = zh-CN ]; then
		ui_print "$1"
	else
		ui_print "$2"
	fi
}

local_echo() {
	if [ $LOCALE = zh-CN ]; then
		echo "$1"
	else
		echo "$2"
	fi
}

if [ $ARCH != arm64 ]; then
	local_print "设备不支持, 非arm64设备" "Only for arm64 device !"
	abort
fi

set_perm_recursive $MODPATH 0 0 0755 0644
set_perm $MODPATH/SimpleScheduler 0 0 0755
CONFIGPATH="/data/adb/SimpleScheduler/"
mkdir -p $CONFIGPATH
cp $MODPATH/config.toml $CONFIFPATH/config.toml
set_perm_recursive $CONFIGPATH 0 0 0755 0644
set_perm $CONFIGPATH/config.toml 0 0 0755
local_print "配置文件夹：$CONFIFPATH/config.toml" "Configuration folder: $CONFIFPATH/config.toml"

