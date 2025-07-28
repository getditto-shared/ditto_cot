//
//  Copyright © 2021 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/DITIdentity.h>
#import <DittoObjC/dittoffi.h>

@class _DITLoginProvider;

NS_ASSUME_NONNULL_BEGIN

@interface DITIdentity ()

// This only gets temporarily cached here before being set as the delegate on the DITAuthenticator
@property (nonatomic, readwrite, nullable) id<DITAuthenticationDelegate> authenticationDelegate;

- (instancetype)initOfflinePlaygroundWithAppID:(nullable NSString *)appID
                                  siteIDNumber:(nullable NSNumber *)siteID;
- (instancetype)initSharedKeyWithAppID:(NSString *)appID
                             sharedKey:(NSString *)sharedKey
                          siteIDNumber:(nullable NSNumber *)siteID;

- (CIdentityConfig_t *)buildIdentityConfig;

@end

NS_ASSUME_NONNULL_END
