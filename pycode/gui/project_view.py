# -*- coding: utf-8 -*-

################################################################################
## Form generated from reading UI file 'project_view.ui'
##
## Created by: Qt User Interface Compiler version 6.7.2
##
## WARNING! All changes made in this file will be lost when recompiling UI file!
################################################################################

from PySide6.QtCore import (QCoreApplication, QDate, QDateTime, QLocale,
    QMetaObject, QObject, QPoint, QRect,
    QSize, QTime, QUrl, Qt)
from PySide6.QtGui import (QBrush, QColor, QConicalGradient, QCursor,
    QFont, QFontDatabase, QGradient, QIcon,
    QImage, QKeySequence, QLinearGradient, QPainter,
    QPalette, QPixmap, QRadialGradient, QTransform)
from PySide6.QtWidgets import (QApplication, QGridLayout, QLabel, QListWidget,
    QListWidgetItem, QSizePolicy, QSplitter, QVBoxLayout,
    QWidget)

class Ui_ProjectView(object):
    def setupUi(self, ProjectView):
        if not ProjectView.objectName():
            ProjectView.setObjectName(u"ProjectView")
        ProjectView.resize(878, 716)
        self.gridLayout = QGridLayout(ProjectView)
        self.gridLayout.setObjectName(u"gridLayout")
        self.splitter = QSplitter(ProjectView)
        self.splitter.setObjectName(u"splitter")
        self.splitter.setOrientation(Qt.Horizontal)
        self.widget = QWidget(self.splitter)
        self.widget.setObjectName(u"widget")
        self.verticalLayout = QVBoxLayout(self.widget)
        self.verticalLayout.setObjectName(u"verticalLayout")
        self.label = QLabel(self.widget)
        self.label.setObjectName(u"label")

        self.verticalLayout.addWidget(self.label)

        self.batches_view = QListWidget(self.widget)
        self.batches_view.setObjectName(u"batches_view")
        sizePolicy = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Expanding)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(self.batches_view.sizePolicy().hasHeightForWidth())
        self.batches_view.setSizePolicy(sizePolicy)
        self.batches_view.setBaseSize(QSize(0, 0))

        self.verticalLayout.addWidget(self.batches_view)

        self.splitter.addWidget(self.widget)
        self.widget_2 = QWidget(self.splitter)
        self.widget_2.setObjectName(u"widget_2")
        self.verticalLayout_2 = QVBoxLayout(self.widget_2)
        self.verticalLayout_2.setObjectName(u"verticalLayout_2")
        self.label_2 = QLabel(self.widget_2)
        self.label_2.setObjectName(u"label_2")

        self.verticalLayout_2.addWidget(self.label_2)

        self.phases_view = QListWidget(self.widget_2)
        self.phases_view.setObjectName(u"phases_view")

        self.verticalLayout_2.addWidget(self.phases_view)

        self.splitter.addWidget(self.widget_2)
        self.widget_3 = QWidget(self.splitter)
        self.widget_3.setObjectName(u"widget_3")
        self.verticalLayout_3 = QVBoxLayout(self.widget_3)
        self.verticalLayout_3.setObjectName(u"verticalLayout_3")
        self.label_3 = QLabel(self.widget_3)
        self.label_3.setObjectName(u"label_3")

        self.verticalLayout_3.addWidget(self.label_3)

        self.datas_view = QListWidget(self.widget_3)
        self.datas_view.setObjectName(u"datas_view")

        self.verticalLayout_3.addWidget(self.datas_view)

        self.splitter.addWidget(self.widget_3)

        self.gridLayout.addWidget(self.splitter, 0, 0, 1, 1)


        self.retranslateUi(ProjectView)

        QMetaObject.connectSlotsByName(ProjectView)
    # setupUi

    def retranslateUi(self, ProjectView):
        ProjectView.setWindowTitle(QCoreApplication.translate("ProjectView", u"Form", None))
        self.label.setText(QCoreApplication.translate("ProjectView", u"Batches", None))
        self.label_2.setText(QCoreApplication.translate("ProjectView", u"Phases", None))
        self.label_3.setText(QCoreApplication.translate("ProjectView", u"Datas", None))
    # retranslateUi

