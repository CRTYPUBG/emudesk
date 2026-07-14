Name:       emudesk
Version:    1.1.9
Release:    0
Summary:    RPM package
License:    GPL-3.0
Requires:   gtk3 libxcb1 libXfixes3 alsa-utils libXtst6 libva2 pam gstreamer-plugins-base gstreamer-plugin-pipewire
Recommends: libayatana-appindicator3-1 xdotool

# https://docs.fedoraproject.org/en-US/packaging-guidelines/Scriptlets/

%description
The best open-source remote desktop client software, written in Rust.

%prep
# we have no source, so nothing here

%build
# we have no source, so nothing here

%global __python %{__python3}

%install
mkdir -p %{buildroot}/usr/bin/
mkdir -p %{buildroot}/usr/share/emudesk/
mkdir -p %{buildroot}/usr/share/emudesk/files/
mkdir -p %{buildroot}/usr/share/icons/hicolor/256x256/apps/
mkdir -p %{buildroot}/usr/share/icons/hicolor/scalable/apps/
install -m 755 $HBB/target/release/emudesk %{buildroot}/usr/bin/emudesk
install $HBB/libsciter-gtk.so %{buildroot}/usr/share/emudesk/libsciter-gtk.so
install $HBB/res/emudesk.service %{buildroot}/usr/share/emudesk/files/
install $HBB/res/128x128@2x.png %{buildroot}/usr/share/icons/hicolor/256x256/apps/emudesk.png
install $HBB/res/scalable.svg %{buildroot}/usr/share/icons/hicolor/scalable/apps/emudesk.svg
install $HBB/res/emudesk.desktop %{buildroot}/usr/share/emudesk/files/
install $HBB/res/emudesk-link.desktop %{buildroot}/usr/share/emudesk/files/

%files
/usr/bin/emudesk
/usr/share/emudesk/libsciter-gtk.so
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
    rm /usr/share/applications/emudesk.desktop || true
    rm /usr/share/applications/emudesk-link.desktop || true
    update-desktop-database
  ;;
  1)
    # for upgrade
  ;;
esac
