
#include "qffi.hpp"
#include <type_traits>

// Helpers

template<typename T, typename ...Args>
static void qffi_call_ctor(T* ptr, Args&&... args) {
  ::new(ptr) T(std::forward<Args>(args)...);
}

template<typename T>
static void qffi_call_default_ctor(T* ptr) {
  ::new(ptr) T();
}

template<typename T>
static void qffi_call_dtor(T* ptr) {
  ptr->~T();
}

// Helpers End

extern "C" {



// QString

static_assert(alignof(QString) == alignof(Qffi_QString), "Alignment of QString incompatible");
static_assert(sizeof(QString) == sizeof(Qffi_QString), "Size of QString incompatible");


void qffi_QString_init(QString* self) {
    ::qffi_call_default_ctor((QString*)self);
}


void qffi_QString_clone(QString const* self, QString* new_) {
    new ((QString*)new_) QString(*(QString const*)self);
}

bool qffi_QString_equals(QString const* self, QString const* other) {
    return *((QString const*)self) == *((QString const*)other);
}


int qffi_QString_size(QString const* _self) {
    auto* self = (QString const*) _self;
    return self->size();
}

bool qffi_QString_isNull(QString const* _self) {
    auto* self = (QString const*) _self;
    return self->isNull();
}

void qffi_QString_fromUtf8(const char* data, int size, QString* result) {
    
    qffi_call_ctor(result, QString::fromUtf8(data, size));
}

void qffi_QString_fromUtf16(const unsigned short* data, int size, QString* result) {
    
    qffi_call_ctor(result, QString::fromUtf16(data, size));
}

void qffi_QString_fromUtf16Unchecked(const unsigned short* data, int size, QString* result) {
    
    qffi_call_ctor(result, (const QChar*)(data), size);
}

void qffi_QString_toUtf8(QString const* _self, QByteArray* result) {
    auto* self = (QString const*) _self;
    qffi_call_ctor(result, self->toUtf8());
}

const unsigned short* qffi_QString_utf16(QString const* _self, int* len) {
    auto* self = (QString const*) _self;
    *len = self->size();
    return (const unsigned short*)self->constData();
}

int qffi_QString_compare(QString const* _self, const QString* other) {
    auto* self = (QString const*) _self;
    return self->compare(*other);
}


// QByteArray

static_assert(alignof(QByteArray) == alignof(Qffi_QByteArray), "Alignment of QByteArray incompatible");
static_assert(sizeof(QByteArray) == sizeof(Qffi_QByteArray), "Size of QByteArray incompatible");


void qffi_QByteArray_init(QByteArray* self) {
    ::qffi_call_default_ctor((QByteArray*)self);
}


void qffi_QByteArray_clone(QByteArray const* self, QByteArray* new_) {
    new ((QByteArray*)new_) QByteArray(*(QByteArray const*)self);
}

bool qffi_QByteArray_equals(QByteArray const* self, QByteArray const* other) {
    return *((QByteArray const*)self) == *((QByteArray const*)other);
}


void qffi_QByteArray_fromData(const char* data, int len, QByteArray* result) {
    
    qffi_call_ctor(result, data, len);
}

const char* qffi_QByteArray_data(QByteArray const* _self, int* len) {
    auto* self = (QByteArray const*) _self;
    *len = self->size();
    return self->constData();
}

int qffi_QByteArray_compare(QByteArray const* _self, const QByteArray* other) {
    auto* self = (QByteArray const*) _self;
    return int(*self < *other) - int(*self > *other);
}


// QUrl

static_assert(alignof(QUrl) == alignof(Qffi_QUrl), "Alignment of QUrl incompatible");
static_assert(sizeof(QUrl) == sizeof(Qffi_QUrl), "Size of QUrl incompatible");


void qffi_QUrl_init(QUrl* self) {
    ::qffi_call_default_ctor((QUrl*)self);
}


void qffi_QUrl_clone(QUrl const* self, QUrl* new_) {
    new ((QUrl*)new_) QUrl(*(QUrl const*)self);
}

bool qffi_QUrl_equals(QUrl const* self, QUrl const* other) {
    return *((QUrl const*)self) == *((QUrl const*)other);
}

signed char qffi_QUrl_ord(QUrl const* _self, QUrl const* _other) {
    auto* self = (QUrl const*) _self;
    auto* other = (QUrl const*) _other;
    return int(*other < *self) - int(*self < *other);
}

void qffi_QUrl_fromString(const QString* value, QUrl* result) {
    
    qffi_call_ctor(result, *value);
}

void qffi_QUrl_fromLocalFile(const QString* value, QUrl* result) {
    
    qffi_call_ctor(result, QUrl::fromLocalFile(*value));
}

void qffi_QUrl_debug(QUrl const* _self, QString* out) {
    auto* self = (QUrl const*) _self;
    new (out) QString();
    QDebug(out).nospace() << *self;
}


// QObject
QObject* qffi_QObject_init(QObject* parent) {
    return (QObject*)new QObject(parent);
}





bool qffi_QObject_inherits(QObject const* _self, const char* class_name) {
    auto* self = (QObject const*) _self;
    return self->inherits(class_name);
}

void qffi_QObject_deleteLater(QObject * _self) {
    auto* self = (QObject *) _self;
    self->deleteLater();
}

const QMetaObject* qffi_QObject_metaObject(QObject const* _self) {
    auto* self = (QObject const*) _self;
    return self->metaObject();
}

void qffi_QObject_moveToThread(QObject * _self, QThread* targetThread) {
    auto* self = (QObject *) _self;
    self->moveToThread(targetThread);
}

void qffi_QObject_connect(QObject const* _self, const char* signal, const QObject* receiver, const char* method, int type_, QMetaObjectConnection* result) {
    auto* self = (QObject const*) _self;
    qffi_call_ctor((QMetaObject::Connection*)result,
      self->connect(self, signal, receiver, method, Qt::ConnectionType(type_)));
}

bool qffi_QObject_disconnectConnection(const QMetaObjectConnection* connection) {
    
    return QObject::disconnect(*(const QMetaObject::Connection*)connection);
}

bool qffi_QObject_disconnect2(QObject const* _self, const QObject* receiver, const char* method) {
    auto* self = (QObject const*) _self;
    return self->disconnect(receiver, method);
}

bool qffi_QObject_disconnect3(QObject const* _self, const char* signal, const QObject* receiver, const char* method) {
    auto* self = (QObject const*) _self;
    return self->disconnect(signal, receiver, method);
}

void qffi_QObject_destroy(QObject * _self) {
    auto* self = (QObject *) _self;
    qffi_call_dtor(self);
}


// QGenericArgument






// QGenericReturnArgument






// QMetaObject





const char* qffi_QMetaObject_className(QMetaObject const* _self) {
    auto* self = (QMetaObject const*) _self;
    return self->className();
}

int qffi_QMetaObject_propertyCount(QMetaObject const* _self) {
    auto* self = (QMetaObject const*) _self;
    return self->propertyCount();
}

int qffi_QMetaObject_propertyOffset(QMetaObject const* _self) {
    auto* self = (QMetaObject const*) _self;
    return self->propertyOffset();
}

QMetaProperty qffi_QMetaObject_property(QMetaObject const* _self, int index) {
    auto* self = (QMetaObject const*) _self;
    return self->property(index);
}

bool qffi_QMetaObject_invokeMethod(QObject* obj, const char* member, int ty, const QGenericArgument* args) {
    
    return QMetaObject::invokeMethod(
      obj, member, Qt::ConnectionType(ty), args[0], args[1], args[2], args[3], args[4], args[5],
      args[6], args[7], args[8], args[9]);
}

bool qffi_QMetaObject_invokeMethodAndReturn(QObject* obj, const char* member, int ty, const QGenericReturnArgument* ret, const QGenericArgument* args) {
    
    return QMetaObject::invokeMethod(
      obj, member, Qt::ConnectionType(ty), *ret, args[0], args[1], args[2], args[3], args[4], args[5],
      args[6], args[7], args[8], args[9]);
}


// QMetaMethod

static_assert(alignof(QMetaMethod) == alignof(Qffi_QMetaMethod), "Alignment of QMetaMethod incompatible");
static_assert(sizeof(QMetaMethod) == sizeof(Qffi_QMetaMethod), "Size of QMetaMethod incompatible");static_assert(std::is_trivially_destructible<QMetaMethod>::value, "QMetaMethod is not trivially destructible");



void qffi_QMetaMethod_init(QMetaMethod* self) {
    ::qffi_call_default_ctor((QMetaMethod*)self);
}

void qffi_QMetaMethod_destroy(QMetaMethod* self) {
    ::qffi_call_dtor((QMetaMethod*)self);
}


bool qffi_QMetaMethod_equals(QMetaMethod const* self, QMetaMethod const* other) {
    return *((QMetaMethod const*)self) == *((QMetaMethod const*)other);
}



// QMetaEnum

static_assert(alignof(QMetaEnum) == alignof(Qffi_QMetaEnum), "Alignment of QMetaEnum incompatible");
static_assert(sizeof(QMetaEnum) == sizeof(Qffi_QMetaEnum), "Size of QMetaEnum incompatible");


void qffi_QMetaEnum_init(QMetaEnum* self) {
    ::qffi_call_default_ctor((QMetaEnum*)self);
}






// QMetaProperty

static_assert(alignof(QMetaProperty) == alignof(Qffi_QMetaProperty), "Alignment of QMetaProperty incompatible");
static_assert(sizeof(QMetaProperty) == sizeof(Qffi_QMetaProperty), "Size of QMetaProperty incompatible");static_assert(std::is_trivially_destructible<QMetaProperty>::value, "QMetaProperty is not trivially destructible");



void qffi_QMetaProperty_init(QMetaProperty* self) {
    ::qffi_call_default_ctor((QMetaProperty*)self);
}

void qffi_QMetaProperty_destroy(QMetaProperty* self) {
    ::qffi_call_dtor((QMetaProperty*)self);
}




bool qffi_QMetaProperty_hasNotifySignal(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->hasNotifySignal();
}

bool qffi_QMetaProperty_isConstant(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->isConstant();
}

bool qffi_QMetaProperty_isDesignable(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->isDesignable();
}

bool qffi_QMetaProperty_isEnumType(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->isEnumType();
}

bool qffi_QMetaProperty_isFinal(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->isFinal();
}

bool qffi_QMetaProperty_isFlagType(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->isFlagType();
}

bool qffi_QMetaProperty_isReadable(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->isReadable();
}

bool qffi_QMetaProperty_isResettable(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->isResettable();
}

bool qffi_QMetaProperty_isScriptable(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->isScriptable();
}

bool qffi_QMetaProperty_isStored(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->isStored();
}

bool qffi_QMetaProperty_isUser(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->isUser();
}

bool qffi_QMetaProperty_isWritable(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->isWritable();
}

const char* qffi_QMetaProperty_name(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->name();
}

QMetaMethod qffi_QMetaProperty_notifySignal(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->notifySignal();
}

int qffi_QMetaProperty_notifySignalIndex(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->notifySignalIndex();
}

int qffi_QMetaProperty_propertyIndex(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->propertyIndex();
}

void qffi_QMetaProperty_read(QMetaProperty const* _self, const QObject* object, QVariant* result) {
    auto* self = (QMetaProperty const*) _self;
    qffi_call_ctor(result, self->read(object));
}

void qffi_QMetaProperty_readOnGadget(QMetaProperty const* _self, const void* gadget, QVariant* result) {
    auto* self = (QMetaProperty const*) _self;
    qffi_call_ctor(result, self->readOnGadget(gadget));
}

bool qffi_QMetaProperty_reset(QMetaProperty const* _self, QObject* object) {
    auto* self = (QMetaProperty const*) _self;
    return self->reset(object);
}

bool qffi_QMetaProperty_resetOnGadget(QMetaProperty const* _self, void* gadget) {
    auto* self = (QMetaProperty const*) _self;
    return self->resetOnGadget(gadget);
}

int qffi_QMetaProperty_revision(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->revision();
}

const char* qffi_QMetaProperty_typeName(QMetaProperty const* _self) {
    auto* self = (QMetaProperty const*) _self;
    return self->typeName();
}

bool qffi_QMetaProperty_write(QMetaProperty const* _self, QObject* object, const QVariant* value) {
    auto* self = (QMetaProperty const*) _self;
    return self->write(object, *value);
}

bool qffi_QMetaProperty_writeOnGadget(QMetaProperty const* _self, void* gadget, const QVariant* value) {
    auto* self = (QMetaProperty const*) _self;
    return self->writeOnGadget(gadget, *value);
}


// QMetaObjectConnection

static_assert(alignof(QMetaObject::Connection) == alignof(QMetaObjectConnection), "Alignment of QMetaObject::Connection incompatible");
static_assert(sizeof(QMetaObject::Connection) == sizeof(QMetaObjectConnection), "Size of QMetaObject::Connection incompatible");


void qffi_QMetaObjectConnection_init(QMetaObjectConnection* self) {
    ::qffi_call_default_ctor((QMetaObject::Connection*)self);
}


void qffi_QMetaObjectConnection_clone(QMetaObjectConnection const* self, QMetaObjectConnection* new_) {
    new ((QMetaObject::Connection*)new_) QMetaObject::Connection(*(QMetaObject::Connection const*)self);
}



bool qffi_QMetaObjectConnection_isValid(QMetaObjectConnection const* _self) {
    auto* self = (QMetaObject::Connection const*) _self;
    return *self;
}


// QVariant

static_assert(alignof(QVariant) == alignof(Qffi_QVariant), "Alignment of QVariant incompatible");
static_assert(sizeof(QVariant) == sizeof(Qffi_QVariant), "Size of QVariant incompatible");


void qffi_QVariant_init(QVariant* self) {
    ::qffi_call_default_ctor((QVariant*)self);
}


void qffi_QVariant_clone(QVariant const* self, QVariant* new_) {
    new ((QVariant*)new_) QVariant(*(QVariant const*)self);
}

bool qffi_QVariant_equals(QVariant const* self, QVariant const* other) {
    return *((QVariant const*)self) == *((QVariant const*)other);
}

signed char qffi_QVariant_ord(QVariant const* _self, QVariant const* _other) {
    auto* self = (QVariant const*) _self;
    auto* other = (QVariant const*) _other;
    return int(*other < *self) - int(*self < *other);
}

bool qffi_QVariant_isValid(QVariant const* _self) {
    auto* self = (QVariant const*) _self;
    return self->isValid();
}

bool qffi_QVariant_isNull(QVariant const* _self) {
    auto* self = (QVariant const*) _self;
    return self->isNull();
}

void qffi_QVariant_from_int(int value, QVariant* result) {
    
    qffi_call_ctor(result, value);
}

void qffi_QVariant_from_uint(unsigned int value, QVariant* result) {
    
    qffi_call_ctor(result, value);
}

void qffi_QVariant_from_int64(long long value, QVariant* result) {
    
    qffi_call_ctor(result, value);
}

void qffi_QVariant_from_uint64(unsigned long long value, QVariant* result) {
    
    qffi_call_ctor(result, value);
}

void qffi_QVariant_from_bool(bool value, QVariant* result) {
    
    qffi_call_ctor(result, value);
}

void qffi_QVariant_from_float(float value, QVariant* result) {
    
    qffi_call_ctor(result, value);
}

void qffi_QVariant_from_double(double value, QVariant* result) {
    
    qffi_call_ctor(result, value);
}

void qffi_QVariant_fromByteArray(const QByteArray* value, QVariant* result) {
    
    qffi_call_ctor(result, *value);
}

void qffi_QVariant_fromString(const QString* value, QVariant* result) {
    
    qffi_call_ctor(result, *value);
}

void qffi_QVariant_fromUtf8(const char* data, int size, QVariant* result) {
    
    qffi_call_ctor(result, QString::fromUtf8(data, size));
}

int qffi_QVariant_toInt(QVariant const* _self, bool* ok) {
    auto* self = (QVariant const*) _self;
    return self->toInt(ok);
}

unsigned int qffi_QVariant_toUInt(QVariant const* _self, bool* ok) {
    auto* self = (QVariant const*) _self;
    return self->toUInt(ok);
}

long long qffi_QVariant_toLongLong(QVariant const* _self, bool* ok) {
    auto* self = (QVariant const*) _self;
    return self->toLongLong(ok);
}

unsigned long long qffi_QVariant_toULongLong(QVariant const* _self, bool* ok) {
    auto* self = (QVariant const*) _self;
    return self->toULongLong(ok);
}

float qffi_QVariant_toFloat(QVariant const* _self, bool* ok) {
    auto* self = (QVariant const*) _self;
    return self->toFloat(ok);
}

double qffi_QVariant_toDouble(QVariant const* _self, bool* ok) {
    auto* self = (QVariant const*) _self;
    return self->toDouble(ok);
}

bool qffi_QVariant_toBool(QVariant const* _self) {
    auto* self = (QVariant const*) _self;
    return self->toBool();
}

void qffi_QVariant_toByteArray(QVariant const* _self, QByteArray* result) {
    auto* self = (QVariant const*) _self;
    qffi_call_ctor(result, self->toByteArray());
}

void qffi_QVariant_toString(QVariant const* _self, QString* result) {
    auto* self = (QVariant const*) _self;
    qffi_call_ctor(result, self->toString());
}

void qffi_QVariant_toUtf8(QVariant const* _self, QByteArray* result) {
    auto* self = (QVariant const*) _self;
    qffi_call_ctor(result, self->toString().toUtf8());
}

void qffi_QVariant_debug(QVariant const* _self, QByteArray* result) {
    auto* self = (QVariant const*) _self;
    QString tmp;
    QDebug(&tmp).nospace() << *self;
    qffi_call_ctor(result, tmp.toUtf8());
}


// QTimer
QTimer* qffi_QTimer_init(QObject* parent) {
    return (QTimer*)new QTimer(parent);
}





bool qffi_QTimer_isActive(QTimer const* _self) {
    auto* self = (QTimer const*) _self;
    return self->isActive();
}

int qffi_QTimer_interval(QTimer const* _self) {
    auto* self = (QTimer const*) _self;
    return self->interval();
}

void qffi_QTimer_setInterval(QTimer * _self, int value) {
    auto* self = (QTimer *) _self;
    self->setInterval(value);
}

int qffi_QTimer_remainingTime(QTimer const* _self) {
    auto* self = (QTimer const*) _self;
    return self->remainingTime();
}

bool qffi_QTimer_isSingleShot(QTimer const* _self) {
    auto* self = (QTimer const*) _self;
    return self->isSingleShot();
}

void qffi_QTimer_setSingleShot(QTimer * _self, bool value) {
    auto* self = (QTimer *) _self;
    return self->setSingleShot(value);
}

int qffi_QTimer_timerType(QTimer const* _self) {
    auto* self = (QTimer const*) _self;
    return self->timerType();
}

void qffi_QTimer_setTimerType(QTimer * _self, int value) {
    auto* self = (QTimer *) _self;
    self->setTimerType(static_cast<Qt::TimerType>(value));
}

void qffi_QTimer_start(QTimer * _self) {
    auto* self = (QTimer *) _self;
    self->start();
}

void qffi_QTimer_startWithInterval(QTimer * _self, int interval) {
    auto* self = (QTimer *) _self;
    self->start(interval);
}

void qffi_QTimer_stop(QTimer * _self) {
    auto* self = (QTimer *) _self;
    self->stop();
}


// QCoreApplication





int qffi_QCoreApplication_exec() {
    
    return QCoreApplication::exec();
}

QCoreApplication* qffi_QCoreApplication_init(int* argc, char const** argv) {
    
    return new QCoreApplication(*argc, (char**)argv);
}


// QGuiApplication





int qffi_QGuiApplication_exec() {
    
    return QGuiApplication::exec();
}

QGuiApplication* qffi_QGuiApplication_init(int* argc, char const** argv) {
    
    return new QGuiApplication(*argc, (char**)argv);
}


// QHashIntQByteArray

static_assert(alignof(QHash<int, QByteArray>) == alignof(QHashIntQByteArray), "Alignment of QHash<int, QByteArray> incompatible");
static_assert(sizeof(QHash<int, QByteArray>) == sizeof(QHashIntQByteArray), "Size of QHash<int, QByteArray> incompatible");


void qffi_QHashIntQByteArray_init(QHashIntQByteArray* self) {
    ::qffi_call_default_ctor((QHash<int, QByteArray>*)self);
}


void qffi_QHashIntQByteArray_clone(QHashIntQByteArray const* self, QHashIntQByteArray* new_) {
    new ((QHash<int, QByteArray>*)new_) QHash<int, QByteArray>(*(QHash<int, QByteArray> const*)self);
}

bool qffi_QHashIntQByteArray_equals(QHashIntQByteArray const* self, QHashIntQByteArray const* other) {
    return *((QHash<int, QByteArray> const*)self) == *((QHash<int, QByteArray> const*)other);
}


int qffi_QHashIntQByteArray_size(QHashIntQByteArray const* _self) {
    auto* self = (QHash<int, QByteArray> const*) _self;
    return self->size();
}

void qffi_QHashIntQByteArray_insert(QHashIntQByteArray * _self, const int* key, const QByteArray* value) {
    auto* self = (QHash<int, QByteArray> *) _self;
    self->insert(*key, *value);
}


// QThread






// QObjectList

static_assert(alignof(QObjectList) == alignof(Qffi_QObjectList), "Alignment of QObjectList incompatible");
static_assert(sizeof(QObjectList) == sizeof(Qffi_QObjectList), "Size of QObjectList incompatible");


void qffi_QObjectList_init(QObjectList* self) {
    ::qffi_call_default_ctor((QObjectList*)self);
}


void qffi_QObjectList_clone(QObjectList const* self, QObjectList* new_) {
    new ((QObjectList*)new_) QObjectList(*(QObjectList const*)self);
}

bool qffi_QObjectList_equals(QObjectList const* self, QObjectList const* other) {
    return *((QObjectList const*)self) == *((QObjectList const*)other);
}


int qffi_QObjectList_size(QObjectList const* _self) {
    auto* self = (QObjectList const*) _self;
    return self->size();
}

QObject* const* qffi_QObjectList_asSlice(QObjectList const* _self, int* size) {
    auto* self = (QObjectList const*) _self;
    *size = self->size(); if (size == 0) { return nullptr; } else { return & self->front(); }
}

void qffi_QObjectList_append(QObjectList * _self, QObject* const* item) {
    auto* self = (QObjectList *) _self;
    self->append(*item);
}

void qffi_QObjectList_appendList(QObjectList * _self, QList<QObject*> const* item) {
    auto* self = (QObjectList *) _self;
    self->append(*item);
}

void qffi_QObjectList_appendSlice(QObjectList * _self, QObject* const* items, int size) {
    auto* self = (QObjectList *) _self;
    self->reserve(self->size() + size); for (int i = 0; i < size; ++i) { self->push_back(items[i]); }
}

void qffi_QObjectList_reserveAdditional(QObjectList * _self, int additional) {
    auto* self = (QObjectList *) _self;
    self->reserve(self->size() + additional);
}


// QStringList

static_assert(alignof(QStringList) == alignof(Qffi_QStringList), "Alignment of QStringList incompatible");
static_assert(sizeof(QStringList) == sizeof(Qffi_QStringList), "Size of QStringList incompatible");


void qffi_QStringList_init(QStringList* self) {
    ::qffi_call_default_ctor((QStringList*)self);
}


void qffi_QStringList_clone(QStringList const* self, QStringList* new_) {
    new ((QStringList*)new_) QStringList(*(QStringList const*)self);
}

bool qffi_QStringList_equals(QStringList const* self, QStringList const* other) {
    return *((QStringList const*)self) == *((QStringList const*)other);
}


int qffi_QStringList_size(QStringList const* _self) {
    auto* self = (QStringList const*) _self;
    return self->size();
}

QString const* qffi_QStringList_asSlice(QStringList const* _self, int* size) {
    auto* self = (QStringList const*) _self;
    *size = self->size(); if (size == 0) { return nullptr; } else { return & self->front(); }
}

void qffi_QStringList_append(QStringList * _self, QString const* item) {
    auto* self = (QStringList *) _self;
    self->append(*item);
}

void qffi_QStringList_appendList(QStringList * _self, QList<QString> const* item) {
    auto* self = (QStringList *) _self;
    self->append(*item);
}

void qffi_QStringList_appendSlice(QStringList * _self, QString const* items, int size) {
    auto* self = (QStringList *) _self;
    self->reserve(self->size() + size); for (int i = 0; i < size; ++i) { self->push_back(items[i]); }
}

void qffi_QStringList_reserveAdditional(QStringList * _self, int additional) {
    auto* self = (QStringList *) _self;
    self->reserve(self->size() + additional);
}

}