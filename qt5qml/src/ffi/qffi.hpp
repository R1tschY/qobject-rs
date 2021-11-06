#ifndef BINDGEN
    #include <QString>
    #include <QByteArray>
    #include <QUrl>
    #include <QObject>
    #include <QGenericArgument>
    #include <QGenericReturnArgument>
    #include <QMetaObject>
    #include <QMetaMethod>
    #include <QMetaEnum>
    #include <QMetaProperty>
    #include <QMetaObject>
    #include <QVariant>
    #include <QTimer>
    #include <QCoreApplication>
    #include <QGuiApplication>
    #include <QHash>
    #include <QThread>
    #include <QDebug>
#endif

#ifdef BINDGEN
#define QFFI_CLASSNAME(cls) cls
#else
#define QFFI_CLASSNAME(cls) Qffi_ ## cls
#endif

class QFFI_CLASSNAME(QString) {
    void* __d;
};

class QFFI_CLASSNAME(QByteArray) {
    void* __d;
};

class QFFI_CLASSNAME(QUrl) {
    void* __d;
};

class QFFI_CLASSNAME(QObject);

class QFFI_CLASSNAME(QGenericArgument) {
    const void* data;
    const char* name;
};

class QFFI_CLASSNAME(QGenericReturnArgument) {
    const void* data;
    const char* name;
};

class QFFI_CLASSNAME(QMetaObject);

class QFFI_CLASSNAME(QMetaMethod) {
    const QMetaObject* __mobj;
    unsigned int __handle;
};

class QFFI_CLASSNAME(QMetaEnum) {
    const QMetaObject* __mobj;
    unsigned int __handle;
};

class QFFI_CLASSNAME(QMetaProperty) {
    const QMetaObject* __mobj;
    unsigned int __handle;
    int __idx;
    QMetaEnum __menum;
};

class QMetaObjectConnection {
    void* __d;
};

class QFFI_CLASSNAME(QVariant) {
    void* __data;
    unsigned int __type;
};

class QFFI_CLASSNAME(QTimer);

class QFFI_CLASSNAME(QCoreApplication);

class QFFI_CLASSNAME(QGuiApplication);

class QHashIntQByteArray {
    void* __d;
};

class QFFI_CLASSNAME(QThread);

#undef QFFI_CLASSNAME

extern "C" {


void qffi_QString_init(QString* self);
void qffi_QString_clone(QString const* self, QString* new_);
bool qffi_QString_equals(QString const* self, QString const* other);
int qffi_QString_size(QString const* self);
bool qffi_QString_isNull(QString const* self);
void qffi_QString_fromUtf8(const char* data, int size, QString* result);
void qffi_QString_fromUtf16(const unsigned short* data, int size, QString* result);
void qffi_QString_fromUtf16Unchecked(const unsigned short* data, int size, QString* result);
void qffi_QString_toUtf8(QString const* self, QByteArray* result);
const unsigned short* qffi_QString_utf16(QString const* self, int* len);
int qffi_QString_compare(QString const* self, const QString* other);


void qffi_QByteArray_init(QByteArray* self);
void qffi_QByteArray_clone(QByteArray const* self, QByteArray* new_);
bool qffi_QByteArray_equals(QByteArray const* self, QByteArray const* other);
void qffi_QByteArray_fromData(const char* data, int len, QByteArray* result);
const char* qffi_QByteArray_data(QByteArray const* self, int* len);
int qffi_QByteArray_compare(QByteArray const* self, const QByteArray* other);


void qffi_QUrl_init(QUrl* self);
void qffi_QUrl_clone(QUrl const* self, QUrl* new_);
bool qffi_QUrl_equals(QUrl const* self, QUrl const* other);
signed char qffi_QUrl_cmp(QUrl const* self, QUrl const* other);
void qffi_QUrl_fromString(const QString* value, QUrl* result);
void qffi_QUrl_fromLocalFile(const QString* value, QUrl* result);
void qffi_QUrl_debug(QUrl const* self, QString* out);


QObject* qffi_QObject_init(QObject* parent);
bool qffi_QObject_inherits(QObject const* self, const char* class_name);
void qffi_QObject_deleteLater(QObject * self);
const QMetaObject* qffi_QObject_metaObject(QObject const* self);
void qffi_QObject_moveToThread(QObject * self, QThread* targetThread);
void qffi_QObject_connect(QObject const* self, const char* signal, const QObject* receiver, const char* method, int type_, QMetaObjectConnection* result);
bool qffi_QObject_disconnectConnection(const QMetaObjectConnection* connection);
bool qffi_QObject_disconnect2(QObject const* self, const QObject* receiver, const char* method);
bool qffi_QObject_disconnect3(QObject const* self, const char* signal, const QObject* receiver, const char* method);
void qffi_QObject_destroy(QObject * self);






const char* qffi_QMetaObject_className(QMetaObject const* self);
int qffi_QMetaObject_propertyCount(QMetaObject const* self);
int qffi_QMetaObject_propertyOffset(QMetaObject const* self);
QMetaProperty qffi_QMetaObject_property(QMetaObject const* self, int index);
bool qffi_QMetaObject_invokeMethod(QObject* obj, const char* member, int ty, const QGenericArgument* args);
bool qffi_QMetaObject_invokeMethodAndReturn(QObject* obj, const char* member, int ty, const QGenericReturnArgument* ret, const QGenericArgument* args);


void qffi_QMetaMethod_init(QMetaMethod* self);
void qffi_QMetaMethod_destroy(QMetaMethod* self);
bool qffi_QMetaMethod_equals(QMetaMethod const* self, QMetaMethod const* other);


void qffi_QMetaEnum_init(QMetaEnum* self);


void qffi_QMetaProperty_init(QMetaProperty* self);
void qffi_QMetaProperty_destroy(QMetaProperty* self);
bool qffi_QMetaProperty_hasNotifySignal(QMetaProperty const* self);
bool qffi_QMetaProperty_isConstant(QMetaProperty const* self);
bool qffi_QMetaProperty_isDesignable(QMetaProperty const* self);
bool qffi_QMetaProperty_isEnumType(QMetaProperty const* self);
bool qffi_QMetaProperty_isFinal(QMetaProperty const* self);
bool qffi_QMetaProperty_isFlagType(QMetaProperty const* self);
bool qffi_QMetaProperty_isReadable(QMetaProperty const* self);
bool qffi_QMetaProperty_isResettable(QMetaProperty const* self);
bool qffi_QMetaProperty_isScriptable(QMetaProperty const* self);
bool qffi_QMetaProperty_isStored(QMetaProperty const* self);
bool qffi_QMetaProperty_isUser(QMetaProperty const* self);
bool qffi_QMetaProperty_isWritable(QMetaProperty const* self);
const char* qffi_QMetaProperty_name(QMetaProperty const* self);
QMetaMethod qffi_QMetaProperty_notifySignal(QMetaProperty const* self);
int qffi_QMetaProperty_notifySignalIndex(QMetaProperty const* self);
int qffi_QMetaProperty_propertyIndex(QMetaProperty const* self);
void qffi_QMetaProperty_read(QMetaProperty const* self, const QObject* object, QVariant* result);
void qffi_QMetaProperty_readOnGadget(QMetaProperty const* self, const void* gadget, QVariant* result);
bool qffi_QMetaProperty_reset(QMetaProperty const* self, QObject* object);
bool qffi_QMetaProperty_resetOnGadget(QMetaProperty const* self, void* gadget);
int qffi_QMetaProperty_revision(QMetaProperty const* self);
const char* qffi_QMetaProperty_typeName(QMetaProperty const* self);
bool qffi_QMetaProperty_write(QMetaProperty const* self, QObject* object, const QVariant* value);
bool qffi_QMetaProperty_writeOnGadget(QMetaProperty const* self, void* gadget, const QVariant* value);


void qffi_QMetaObjectConnection_init(QMetaObjectConnection* self);
void qffi_QMetaObjectConnection_clone(QMetaObjectConnection const* self, QMetaObjectConnection* new_);
bool qffi_QMetaObjectConnection_isValid(QMetaObjectConnection const* self);


void qffi_QVariant_init(QVariant* self);
void qffi_QVariant_clone(QVariant const* self, QVariant* new_);
bool qffi_QVariant_equals(QVariant const* self, QVariant const* other);
signed char qffi_QVariant_cmp(QVariant const* self, QVariant const* other);
bool qffi_QVariant_isValid(QVariant const* self);
bool qffi_QVariant_isNull(QVariant const* self);
void qffi_QVariant_from_int(int value, QVariant* result);
void qffi_QVariant_from_uint(unsigned int value, QVariant* result);
void qffi_QVariant_from_int64(long long value, QVariant* result);
void qffi_QVariant_from_uint64(unsigned long long value, QVariant* result);
void qffi_QVariant_from_bool(bool value, QVariant* result);
void qffi_QVariant_from_float(float value, QVariant* result);
void qffi_QVariant_from_double(double value, QVariant* result);
void qffi_QVariant_fromByteArray(const QByteArray* value, QVariant* result);
void qffi_QVariant_fromString(const QString* value, QVariant* result);
void qffi_QVariant_fromUtf8(const char* data, int size, QVariant* result);
int qffi_QVariant_toInt(QVariant const* self, bool* ok);
unsigned int qffi_QVariant_toUInt(QVariant const* self, bool* ok);
long long qffi_QVariant_toLongLong(QVariant const* self, bool* ok);
unsigned long long qffi_QVariant_toULongLong(QVariant const* self, bool* ok);
float qffi_QVariant_toFloat(QVariant const* self, bool* ok);
double qffi_QVariant_toDouble(QVariant const* self, bool* ok);
bool qffi_QVariant_toBool(QVariant const* self);
void qffi_QVariant_toByteArray(QVariant const* self, QByteArray* result);
void qffi_QVariant_toString(QVariant const* self, QString* result);
void qffi_QVariant_toUtf8(QVariant const* self, QByteArray* result);
void qffi_QVariant_debug(QVariant const* self, QByteArray* result);


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


void qffi_QHashIntQByteArray_init(QHashIntQByteArray* self);
void qffi_QHashIntQByteArray_clone(QHashIntQByteArray const* self, QHashIntQByteArray* new_);
bool qffi_QHashIntQByteArray_equals(QHashIntQByteArray const* self, QHashIntQByteArray const* other);
int qffi_QHashIntQByteArray_size(QHashIntQByteArray const* self);
void qffi_QHashIntQByteArray_insert(QHashIntQByteArray * self, const int* key, const QByteArray* value);





}