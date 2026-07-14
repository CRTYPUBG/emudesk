Name:       emudesk
Version:    1.4.9
Release:    0
Summary:    RPM package
License:    GPL-3.0
URL:        https://emudesk.com
Vendor:     emudesk <info@emudesk.com>
Requires:   gtk3 libxcb libXfixes alsa-lib libva pam gstreamer1-plugins-base
Recommends: libayatana-appindicator-gtk3 libxdo
Provides:   libdesktop_drop_plugin.so()(64bit), libdesktop_multi_window_plugin.so()(64bit), libfile_selector_linux_plugin.so()(64bit), libflutter_custom_cursor_plugin.so()(64bit), libflutter_linux_gtk.so()(64bit), libscreen_retriever_plugin.so()(64bit), libtray_manager_plugin.so()(64bit), liburl_launcher_linux_plugin.so()(64bit), libwindow_manager_plugin.so()(64bit), libwindow_size_plugin.so()(64bit), libtexture_rgba_renderer_plugin.so()(64bit)

# https://docs.fedoraproject.org/en-US/packaging-guidelines/Scriptlets/

%description
The best open-source remote desktop client software, written in Rust.

%prep
# we have no source, so nothing here

%build
# we have no source, so nothing here

# %global __python %{__python3}

%install

mkdir -p "%{buildroot}/usr/share/emudesk" && cp -r ${HBB}/flutter/build/linux/x64/release/bundle/* -t "%{buildroot}/usr/share/emudesk"
mkdir -p "%{buildroot}/usr/bin"
install -Dm 644 $HBB/res/emudesk.service -t "%{buildroot}/usr/share/emudesk/files"
install -Dm 644 $HBB/res/emudesk.desktop -t "%{buildroot}/usr/share/emudesk/files"
install -Dm 644 $HBB/res/emudesk-link.desktop -t "%{buildroot}/usr/share/emudesk/files"
install -Dm 644 $HBB/res/128x128@2x.png "%{buildroot}/usr/share/icons/hicolor/256x256/apps/emudesk.png"
install -Dm 644 $HBB/res/scalable.svg "%{buildroot}/usr/share/icons/hicolor/scalable/apps/emudesk.svg"

%files
/usr/share/emudesk/*
/usr/share/emudesk/files/emudesk.service
/usr/share/icons/hicolor/256x256/apps/emudesk.png
/usr/share/icons/hicolor/scalable/apps/emudesk.svg
/usr/share/emudesk/files/emudesk.desktop
/usr/share/emudesk/files/emudesk-link.desktop

%changelog
# let's skip this for now

%pre
# can do something for centos7
case "$1" in
  1)
    # for install
  ;;
  2)
    # for upgrade
    systemctl stop emudesk || true
  ;;
esac

%post
cp /usr/share/emudesk/files/emudesk.service /etc/systemd/system/emudesk.service
cp /usr/share/emudesk/files/emudesk.desktop /usr/share/applications/
cp /usr/share/emudesk/files/emudesk-link.desktop /usr/share/applications/
ln -sf /usr/share/emudesk/emudesk /usr/bin/emudesk
systemctl daemon-reload
systemctl enable emudesk
systemctl start emudesk
update-desktop-database

%preun
case "$1" in
  0)
    # for uninstall
    systemctl stop emudesk || true
    systemctl disable emudesk || true
    rm /etc/systemd/system/emudesk.service || true
  ;;
  1)
    # for upgrade
  ;;
esac

%postun
case "$1" in
  0)
    # for uninstall
    rm /usr/bin/emudesk || true
    rmdir /usr/lib/emudesk || true
    rmdir /usr/local/emudesk || true
    rmdir /usr/share/emudesk || true
    rm /usr/share/applications/emudesk.desktop || true
    rm /usr/share/applications/emudesk-link.desktop || true
    update-desktop-database
  ;;
  1)
    # for upgrade
    rmdir /usr/lib/emudesk || true
    rmdir /usr/local/emudesk || true
  ;;
esac
