
#include "qffi.hpp"

extern "C" {



// QString

static_assert(alignof(QString) == alignof(_QString), "Alignment of QString incompatible");
static_assert(sizeof(QString) == sizeof(_QString), "Size of QString incompatible");


void qffi_QString_init(QString* self) {
    new (self) QString();
}

void qffi_QString_destroy(QString* self) {
    self->~QString();
}

void qffi_QString_clone(QString const* self, QString* new_) {
    new (new_) QString(*self);
}

bool qffi_QString_equals(QString const* self, QString const* other) {
    return *self == *other;
}


int qffi_QString_size(QString const* self) {
    return self->size();
}

bool qffi_QString_isNull(QString const* self) {
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

void qffi_QString_toUtf8(QString const* self, QByteArray* dest) {
    new (dest) QByteArray(self->toUtf8());
}

const unsigned short* qffi_QString_utf16(QString const* self, int* len) {
    *len = self->size();
    return (const unsigned short*)self->constData();
}

int qffi_QString_compare(QString const* self, const QString* other) {
    return self->compare(*other);
}


// QByteArray

static_assert(alignof(QByteArray) == alignof(_QByteArray), "Alignment of QByteArray incompatible");
static_assert(sizeof(QByteArray) == sizeof(_QByteArray), "Size of QByteArray incompatible");


void qffi_QByteArray_init(QByteArray* self) {
    new (self) QByteArray();
}

void qffi_QByteArray_destroy(QByteArray* self) {
    self->~QByteArray();
}

void qffi_QByteArray_clone(QByteArray const* self, QByteArray* new_) {
    new (new_) QByteArray(*self);
}

bool qffi_QByteArray_equals(QByteArray const* self, QByteArray const* other) {
    return *self == *other;
}


void qffi_QByteArray_fromData(const char* data, int len, QByteArray* dest) {
    new (dest) QByteArray(data, len);
}

const char* qffi_QByteArray_data(QByteArray const* self, int* len) {
    *len = self->size();
    return self->constData();
}

int qffi_QByteArray_compare(QByteArray const* self, const QByteArray* other) {
    return int(*self < *other) - int(*self > *other);
}


// QTimer
QTimer* qffi_QTimer_init(QObject* parent) {
    return new QTimer(parent);
}

void qffi_QTimer_destroy(QTimer* self) {
    delete self;
}




bool qffi_QTimer_isActive(QTimer const* self) {
    return self->isActive();
}

int qffi_QTimer_interval(QTimer const* self) {
    return self->interval();
}

void qffi_QTimer_setInterval(QTimer * self, int value) {
    self->setInterval(value);
}

int qffi_QTimer_remainingTime(QTimer const* self) {
    return self->remainingTime();
}

bool qffi_QTimer_isSingleShot(QTimer const* self) {
    return self->isSingleShot();
}

void qffi_QTimer_setSingleShot(QTimer * self, bool value) {
    return self->setSingleShot(value);
}

int qffi_QTimer_timerType(QTimer const* self) {
    return self->timerType();
}

void qffi_QTimer_setTimerType(QTimer * self, int value) {
    self->setTimerType(static_cast<Qt::TimerType>(value));
}

void qffi_QTimer_start(QTimer * self) {
    self->start();
}

void qffi_QTimer_startWithInterval(QTimer * self, int interval) {
    self->start(interval);
}

void qffi_QTimer_stop(QTimer * self) {
    self->stop();
}

}