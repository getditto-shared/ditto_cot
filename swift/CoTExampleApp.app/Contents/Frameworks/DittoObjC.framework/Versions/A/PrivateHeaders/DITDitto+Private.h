//
//  Copyright © 2020 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/DITDitto.h>

@class _DITDittoHandleWrapper;
@class _DITPresenceManagerV1;
@class _DITPresenceManagerV2;
@class DITPresence;

@class DITTransportHandleWrapper;
@class DITAWDLClientPlatform;
@class DITAWDLServerPlatform;
@class DITBluetoothPlatform;
@class DITMDNSPlatform;

@class DITBluetoothLEConfig;
@class DITLANConfig;
@class DITAWDLConfig;
@class DITTCPListenConfig;
@class DITHTTPListenConfig;
@class DITGlobalConfig;

@class DITIdentityProvider;

@class _DITHistory;
@class _DITStatus;

NS_ASSUME_NONNULL_BEGIN

@interface DITDitto ()
@property (nonatomic, readonly) _DITDittoHandleWrapper *dittoHandleWrapper;

@property (nonatomic, readonly)
    _DITPresenceManagerV1 *presenceV1 __deprecated_msg("Replaced by `presence`.");
@property (nonatomic, readonly)
    _DITPresenceManagerV2 *presenceV2 __deprecated_msg("Replaced by `presence`.");

@property (nonatomic) BOOL transportConditionCallbackRegistered;

@property (nonatomic) _DITStatus *status;

@property (nonatomic, readwrite, nullable) DITIdentityProvider *identityProvider;

@property (nonatomic, readonly) _DITHistory *history;

- (nullable instancetype)initWithIdentity:(DITIdentity *)identity
                   historyTrackingEnabled:(BOOL)historyTrackingEnabled
                     persistenceDirectory:(nullable NSURL *)directory
                               passphrase:(nullable NSString *)passphrase
                                    error:(NSError *_Nullable *)error NS_DESIGNATED_INITIALIZER;

- (void)initSdkVersion;

@end

NS_ASSUME_NONNULL_END
