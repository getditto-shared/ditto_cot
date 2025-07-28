//
// Copyright © 2021 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/DITAuthenticationStatus.h>

NS_ASSUME_NONNULL_BEGIN

@interface DITAuthenticationStatus ()

+ (instancetype)fromFFI:(dittoffi_authentication_status_t *)ffiAuthenticationStatus;

@end

NS_ASSUME_NONNULL_END
