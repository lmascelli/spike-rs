# -*- coding: utf-8 -*-

################################################################################
## Form generated from reading UI file 'main.ui'
##
## Created by: Qt User Interface Compiler version 6.7.2
##
## WARNING! All changes made in this file will be lost when recompiling UI file!
################################################################################

from PySide6.QtCore import (QCoreApplication, QDate, QDateTime, QLocale,
    QMetaObject, QObject, QPoint, QRect,
    QSize, QTime, QUrl, Qt)
from PySide6.QtGui import (QAction, QBrush, QColor, QConicalGradient,
    QCursor, QFont, QFontDatabase, QGradient,
    QIcon, QImage, QKeySequence, QLinearGradient,
    QPainter, QPalette, QPixmap, QRadialGradient,
    QTransform)
from PySide6.QtWidgets import (QApplication, QGridLayout, QLabel, QMainWindow,
    QMenu, QMenuBar, QSizePolicy, QStatusBar,
    QTabWidget, QToolBar, QWidget)

class Ui_MainWindow(object):
    def setupUi(self, MainWindow):
        if not MainWindow.objectName():
            MainWindow.setObjectName(u"MainWindow")
        MainWindow.resize(800, 600)
        sizePolicy = QSizePolicy(QSizePolicy.Policy.Preferred, QSizePolicy.Policy.Preferred)
        sizePolicy.setHorizontalStretch(0)
        sizePolicy.setVerticalStretch(0)
        sizePolicy.setHeightForWidth(MainWindow.sizePolicy().hasHeightForWidth())
        MainWindow.setSizePolicy(sizePolicy)
        MainWindow.setAutoFillBackground(False)
        self.action_Open_Project = QAction(MainWindow)
        self.action_Open_Project.setObjectName(u"action_Open_Project")
        self.actionExit = QAction(MainWindow)
        self.actionExit.setObjectName(u"actionExit")
        self.action_Add_batch = QAction(MainWindow)
        self.action_Add_batch.setObjectName(u"action_Add_batch")
        self.action_CloseTab = QAction(MainWindow)
        self.action_CloseTab.setObjectName(u"action_CloseTab")
        self.action_New_Project = QAction(MainWindow)
        self.action_New_Project.setObjectName(u"action_New_Project")
        self.centralwidget = QWidget(MainWindow)
        self.centralwidget.setObjectName(u"centralwidget")
        self.gridLayout = QGridLayout(self.centralwidget)
        self.gridLayout.setObjectName(u"gridLayout")
        self.w_tab = QTabWidget(self.centralwidget)
        self.w_tab.setObjectName(u"w_tab")
        self.w_tab.setTabPosition(QTabWidget.North)
        self.w_tab.setTabShape(QTabWidget.Rounded)
        self.w_tab.setElideMode(Qt.ElideNone)
        self.w_tab.setDocumentMode(False)
        self.w_tab.setTabsClosable(True)
        self.w_tab.setMovable(True)
        self.Welcome = QWidget()
        self.Welcome.setObjectName(u"Welcome")
        self.gridLayout_2 = QGridLayout(self.Welcome)
        self.gridLayout_2.setObjectName(u"gridLayout_2")
        self.label = QLabel(self.Welcome)
        self.label.setObjectName(u"label")
        self.label.setTextFormat(Qt.MarkdownText)
        self.label.setAlignment(Qt.AlignCenter)

        self.gridLayout_2.addWidget(self.label, 0, 0, 1, 1)

        self.w_tab.addTab(self.Welcome, "")

        self.gridLayout.addWidget(self.w_tab, 0, 0, 1, 1)

        MainWindow.setCentralWidget(self.centralwidget)
        self.menubar = QMenuBar(MainWindow)
        self.menubar.setObjectName(u"menubar")
        self.menubar.setGeometry(QRect(0, 0, 800, 22))
        self.menuFIle = QMenu(self.menubar)
        self.menuFIle.setObjectName(u"menuFIle")
        self.menu = QMenu(self.menubar)
        self.menu.setObjectName(u"menu")
        self.menuProject = QMenu(self.menubar)
        self.menuProject.setObjectName(u"menuProject")
        MainWindow.setMenuBar(self.menubar)
        self.statusbar = QStatusBar(MainWindow)
        self.statusbar.setObjectName(u"statusbar")
        MainWindow.setStatusBar(self.statusbar)
        self.toolBar = QToolBar(MainWindow)
        self.toolBar.setObjectName(u"toolBar")
        MainWindow.addToolBar(Qt.ToolBarArea.TopToolBarArea, self.toolBar)

        self.menubar.addAction(self.menuFIle.menuAction())
        self.menubar.addAction(self.menuProject.menuAction())
        self.menubar.addAction(self.menu.menuAction())
        self.menuFIle.addAction(self.action_New_Project)
        self.menuFIle.addAction(self.action_Open_Project)
        self.menuFIle.addSeparator()
        self.menuFIle.addAction(self.actionExit)
        self.menuFIle.addSeparator()
        self.menuProject.addAction(self.action_Add_batch)
        self.menuProject.addAction(self.action_CloseTab)
        self.toolBar.addAction(self.action_New_Project)
        self.toolBar.addAction(self.action_Open_Project)
        self.toolBar.addAction(self.action_Add_batch)
        self.toolBar.addAction(self.action_CloseTab)

        self.retranslateUi(MainWindow)

        self.w_tab.setCurrentIndex(0)


        QMetaObject.connectSlotsByName(MainWindow)
    # setupUi

    def retranslateUi(self, MainWindow):
        MainWindow.setWindowTitle(QCoreApplication.translate("MainWindow", u"PyCode", None))
        self.action_Open_Project.setText(QCoreApplication.translate("MainWindow", u"&Open Project", None))
#if QT_CONFIG(shortcut)
        self.action_Open_Project.setShortcut(QCoreApplication.translate("MainWindow", u"Ctrl+O", None))
#endif // QT_CONFIG(shortcut)
        self.actionExit.setText(QCoreApplication.translate("MainWindow", u"&Exit", None))
        self.action_Add_batch.setText(QCoreApplication.translate("MainWindow", u"&Add batch folder", None))
#if QT_CONFIG(shortcut)
        self.action_Add_batch.setShortcut(QCoreApplication.translate("MainWindow", u"Ctrl+B", None))
#endif // QT_CONFIG(shortcut)
        self.action_CloseTab.setText(QCoreApplication.translate("MainWindow", u"&CloseTab", None))
#if QT_CONFIG(shortcut)
        self.action_CloseTab.setShortcut(QCoreApplication.translate("MainWindow", u"Ctrl+W", None))
#endif // QT_CONFIG(shortcut)
        self.action_New_Project.setText(QCoreApplication.translate("MainWindow", u"&New Project", None))
#if QT_CONFIG(shortcut)
        self.action_New_Project.setShortcut(QCoreApplication.translate("MainWindow", u"Ctrl+N", None))
#endif // QT_CONFIG(shortcut)
        self.label.setText(QCoreApplication.translate("MainWindow", u"<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.0//EN\" \"http://www.w3.org/TR/REC-html40/strict.dtd\">\n"
"<html><head><meta name=\"qrichtext\" content=\"1\" /><style type=\"text/css\">\n"
"p, li { white-space: pre-wrap; }\n"
"</style></head><body style=\" font-family:'Sans Serif'; font-size:9pt; font-weight:400; font-style:normal;\">\n"
"<p align=\"center\" style=\" margin-top:12px; margin-bottom:12px; margin-left:0px; margin-right:0px; -qt-block-indent:0; text-indent:0px;\"><span style=\" font-size:26pt;\">PyCode</span></p>\n"
"<p align=\"center\" style=\"-qt-paragraph-type:empty; margin-top:12px; margin-bottom:12px; margin-left:0px; margin-right:0px; -qt-block-indent:0; text-indent:0px; font-size:48pt;\"><br /></p>\n"
"<p align=\"center\" style=\"-qt-paragraph-type:empty; margin-top:12px; margin-bottom:12px; margin-left:0px; margin-right:0px; -qt-block-indent:0; text-indent:0px;\"><br /></p>\n"
"<p align=\"center\" style=\"-qt-paragraph-type:empty; margin-top:12px; margin-bottom:12px; margin-left:0px; margin-ri"
                        "ght:0px; -qt-block-indent:0; text-indent:0px;\"><br /></p>\n"
"<p align=\"center\" style=\" margin-top:12px; margin-bottom:12px; margin-left:0px; margin-right:0px; -qt-block-indent:0; text-indent:0px;\"><span style=\" font-size:11pt; color:#55aa00;\">Ctrl + N			      Create new project</span></p>\n"
"<p align=\"center\" style=\" margin-top:12px; margin-bottom:12px; margin-left:0px; margin-right:0px; -qt-block-indent:0; text-indent:0px;\"><span style=\" font-size:11pt; color:#55aa00;\">Ctrl + O			 Open existing project</span></p></body></html>", None))
        self.w_tab.setTabText(self.w_tab.indexOf(self.Welcome), QCoreApplication.translate("MainWindow", u"Welcome", None))
        self.menuFIle.setTitle(QCoreApplication.translate("MainWindow", u"&File", None))
        self.menu.setTitle(QCoreApplication.translate("MainWindow", u"?", None))
        self.menuProject.setTitle(QCoreApplication.translate("MainWindow", u"Project", None))
        self.toolBar.setWindowTitle(QCoreApplication.translate("MainWindow", u"toolBar", None))
    # retranslateUi

