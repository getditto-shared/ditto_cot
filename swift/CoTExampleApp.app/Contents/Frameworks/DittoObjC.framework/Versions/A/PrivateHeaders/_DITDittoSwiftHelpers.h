//
// Copyright © 2023 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/dittoffi.h>
#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

// WORKAROUND: opaque structs are unusable from Swift. We only use pointers
// to it in Swift, so size doesn't matter. Therefore we define it here to be
// empty to allow Swift to work with it as pointers.
struct dittoffi_query_result_item {};
struct dittoffi_query_result {};
struct dittoffi_panic {};

// WORKAROUND: We are hitting some really weird Swift <> C bridging edge case.
// Calling the DittoFFI C function straight from Swift crashes, but calling
// an exact 1:1 wrapper works. Ham and Daniel have been digging and are
// suspecting some kind of ABI issue with dittoffi. Fix and remove this
// workaround.
dittoffi_result_dittoffi_query_result_ptr_t dittoffi_try_exec_statement_swift(
    CDitto_t const *ditto,
    char const *statement,
    slice_ref_uint8_t args_cbor);

void dittoffi_presence_set_connection_request_handler_swift(
    CDitto_t const *ditto,
    void (^_Nullable handler)(dittoffi_connection_request_t *));

void dittoffi_ditto_set_panic_handler_swift(void (^_Nullable handler)(dittoffi_panic_t *));

void dittoffi_logger_try_export_to_file_async_swift(char const *dest_path,
                                                    void (^completion)(dittoffi_result_uint64_t));

NSException *_Nullable DITCatchObjCException(void(NS_NOESCAPE ^ block)(void));

NSData *_Nullable DITDecompressGzipData(NSData *compressedData);

NS_ASSUME_NONNULL_END
