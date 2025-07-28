//
//  Copyright © 2020 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/DITErrors.h>

#import <DittoObjC/dittoffi.h>
#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

FOUNDATION_EXPORT NSString *const DITFFIErrorKey;

NSError *DITErrorFromFFIError(dittoffi_error_t *ffiError);
NSError *DITErrorFromFFIErrorAssigningToOutErrorIfNeeded(dittoffi_error_t *ffiError,
                                                         NSError *_Nullable *outError);

NS_ASSUME_NONNULL_END
