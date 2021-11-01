
class _QString {
    void* _1;
};
class _QByteArray {
    void* _1;
};
class _QUrl {
    void* _1;
};


#ifdef BINDGEN
class QObject;
typedef _QString QString;
typedef _QByteArray QByteArray;
typedef _QUrl QUrl;
class QObject;
class QTimer;
class QCoreApplication;
class QGuiApplication;
#else
#include <QString>
#include <QByteArray>
#include <QUrl>
#include <QObject>
#include <QTimer>
#include <QCoreApplication>
#include <QGuiApplication>
#include <QDebug>
#endif

extern "C" {


void qffi_QString_init(QString* self);
void qffi_QString_destroy(QString* self);
void qffi_QString_clone(QString const* self, QString* new_);
bool qffi_QString_equals(QString const* self, QString const* other);
int qffi_QString_size(QString const* self);
bool qffi_QString_isNull(QString const* self);
void qffi_QString_fromUtf8(const char* data, int size, QString* dest);
void qffi_QString_fromUtf16(const unsigned short* data, int size, QString* dest);
void qffi_QString_fromUtf16Unchecked(const unsigned short* data, int size, QString* dest);
void qffi_QString_toUtf8(QString const* self, QByteArray* dest);
const unsigned short* qffi_QString_utf16(QString const* self, int* len);
int qffi_QString_compare(QString const* self, const QString* other);


void qffi_QByteArray_init(QByteArray* self);
void qffi_QByteArray_destroy(QByteArray* self);
void qffi_QByteArray_clone(QByteArray const* self, QByteArray* new_);
bool qffi_QByteArray_equals(QByteArray const* self, QByteArray const* other);
void qffi_QByteArray_fromData(const char* data, int len, QByteArray* dest);
const char* qffi_QByteArray_data(QByteArray const* self, int* len);
int qffi_QByteArray_compare(QByteArray const* self, const QByteArray* other);


void qffi_QUrl_init(QUrl* self);
void qffi_QUrl_destroy(QUrl* self);
void qffi_QUrl_clone(QUrl const* self, QUrl* new_);
bool qffi_QUrl_equals(QUrl const* self, QUrl const* other);
signed char qffi_QUrl_cmp(QUrl const* self, QUrl const* other);
void qffi_QUrl_fromString(const QString* value, QUrl* out);
void qffi_QUrl_fromLocalFile(const QString* value, QUrl* out);
void qffi_QUrl_debug(QUrl const* self, QString* out);


void qffi_QObject_destroy(QObject* self);


QTimer* qffi_QTimer_init(QObject* parent);
bool qffi_QTimer_isActive(QTimer const* self);
int qffi_QTimer_interval(QTimer const* self);
void qffi_QTimer_setInterval(QTimer * self, int value);
int qffi_QTimer_remainingTime(QTimer const* self);
bool qffi_QTimer_isSingleShot(QTimer const* self);
void qffi_QTimer_setSingleShot(QTimer * self, bool value);
int qffi_QTimer_timerType(QTimer const* self);
void qffi_QTimer_setTimerType(QTimer * self, int value);
void qffi_QTimer_start(QTimer * self);
void qffi_QTimer_startWithInterval(QTimer * self, int interval);
void qffi_QTimer_stop(QTimer * self);


int qffi_QCoreApplication_exec();
QCoreApplication* qffi_QCoreApplication_init(int* argc, char const** argv);


int qffi_QGuiApplication_exec();
QGuiApplication* qffi_QGuiApplication_init(int* argc, char const** argv);



}