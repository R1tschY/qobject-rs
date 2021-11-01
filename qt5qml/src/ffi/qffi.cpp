
#include "qffi.hpp"

extern "C" {



// QString

static_assert(alignof(QString) == alignof(_QString), "Alignment of QString incompatible");
static_assert(sizeof(QString) == sizeof(_QString), "Size of QString incompatible");


void qffi_QString_init(QString* self) {
    new ((QString*)self) QString();
}

void qffi_QString_destroy(QString* self) {
    ((QString*)self)->~QString();
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

void qffi_QString_fromUtf8(const char* data, int size, QString* dest) {
    
    new (dest) QString(QString::fromUtf8(data, size));
}

void qffi_QString_fromUtf16(const unsigned short* data, int size, QString* dest) {
    
    new (dest) QString(QString::fromUtf16(data, size));
}

void qffi_QString_fromUtf16Unchecked(const unsigned short* data, int size, QString* dest) {
    
    new (dest) QString((const QChar*)(data), size);
}

void qffi_QString_toUtf8(QString const* _self, QByteArray* dest) {
    auto* self = (QString const*) _self;
    new (dest) QByteArray(self->toUtf8());
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

static_assert(alignof(QByteArray) == alignof(_QByteArray), "Alignment of QByteArray incompatible");
static_assert(sizeof(QByteArray) == sizeof(_QByteArray), "Size of QByteArray incompatible");


void qffi_QByteArray_init(QByteArray* self) {
    new ((QByteArray*)self) QByteArray();
}

void qffi_QByteArray_destroy(QByteArray* self) {
    ((QByteArray*)self)->~QByteArray();
}

void qffi_QByteArray_clone(QByteArray const* self, QByteArray* new_) {
    new ((QByteArray*)new_) QByteArray(*(QByteArray const*)self);
}

bool qffi_QByteArray_equals(QByteArray const* self, QByteArray const* other) {
    return *((QByteArray const*)self) == *((QByteArray const*)other);
}


void qffi_QByteArray_fromData(const char* data, int len, QByteArray* dest) {
    
    new (dest) QByteArray(data, len);
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

static_assert(alignof(QUrl) == alignof(_QUrl), "Alignment of QUrl incompatible");
static_assert(sizeof(QUrl) == sizeof(_QUrl), "Size of QUrl incompatible");


void qffi_QUrl_init(QUrl* self) {
    new ((QUrl*)self) QUrl();
}

void qffi_QUrl_destroy(QUrl* self) {
    ((QUrl*)self)->~QUrl();
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

void qffi_QUrl_fromString(const QString* value, QUrl* out) {
    
    new (out) QUrl(*value);
}

void qffi_QUrl_fromLocalFile(const QString* value, QUrl* out) {
    
    new (out) QUrl(QUrl::fromLocalFile(*value));
}

void qffi_QUrl_debug(QUrl const* _self, QString* out) {
    auto* self = (QUrl const*) _self;
    new (out) QString();
    QDebug(out).nospace() << *self;
}


// QObject

void qffi_QObject_destroy(QObject* self) {
    delete (QObject*)self;
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

static_assert(alignof(QHash<int, QByteArray>) == alignof(_QHashIntQByteArray), "Alignment of QHash<int, QByteArray> incompatible");
static_assert(sizeof(QHash<int, QByteArray>) == sizeof(_QHashIntQByteArray), "Size of QHash<int, QByteArray> incompatible");


void qffi_QHashIntQByteArray_init(QHashIntQByteArray* self) {
    new ((QHash<int, QByteArray>*)self) QHash<int, QByteArray>();
}

void qffi_QHashIntQByteArray_destroy(QHashIntQByteArray* self) {
    ((QHash<int, QByteArray>*)self)->~QHash<int, QByteArray>();
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





}