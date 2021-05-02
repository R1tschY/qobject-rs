
struct _QString {
    void* __priv;
};
struct _QByteArray {
    void* __priv;
};


#ifdef BINDGEN
typedef _QString QString;
typedef _QByteArray QByteArray;

#else
#include <QString>
#include <QByteArray>

#endif

extern "C" {

void qffi_QString_init(QString* self);
void qffi_QString_destroy(QString* self);
void qffi_QString_clone(QString const* self, QString* new_);
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
void qffi_QByteArray_clone(QByteArray const* self, QByteArray* new_);
bool qffi_QByteArray_equals(QByteArray const* self, QByteArray const* other);
void qffi_QByteArray_fromData(const char* data, int len, QByteArray* dest);
const char* qffi_QByteArray_data(QByteArray const* self, int* len);
int qffi_QByteArray_compare(QByteArray const* self, const QByteArray* other);




}