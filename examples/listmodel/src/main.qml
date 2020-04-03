import QtQuick 2.9
import QtQuick.Controls 2.2
import qobject_rs.test 1.0

ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: qsTr("Scroll")

    ScrollView {
        anchors.fill: parent

        ListView {
            width: parent.width
            model: TestObject { }
            delegate: ItemDelegate {
                text: name + ": " + description
                width: parent.width
            }
        }
    }
}
