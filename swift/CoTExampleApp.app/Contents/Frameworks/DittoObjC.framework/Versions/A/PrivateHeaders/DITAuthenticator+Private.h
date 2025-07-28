//
//  Copyright © 2021 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/DITAuthenticator.h>

#import <DittoObjC/dittoffi.h>

@class _DITAuthenticationStatusHandler;
@class _DITAuthenticationStatusObserver;
@class _DITLoginProvider;
@class _DITWeakWrapper;

NS_ASSUME_NONNULL_BEGIN

@interface DITAuthenticator () <DITAuthenticationDelegate>

@property (nonatomic, nullable, weak) DITDitto *ditto;
@property (nonatomic, readonly, strong) NSMutableArray<_DITWeakWrapper *> *statusObservers;
@property (nonatomic, readonly, strong) dispatch_queue_t loginQueue;
@property (nonatomic, nullable, readwrite, weak) id<DITAuthenticationDelegate> delegate;
@property (nonatomic, nullable, readwrite) _DITLoginProvider *loginProvider;
@property (nonatomic, nullable, readwrite)
    _DITAuthenticationStatusHandler *authenticationStatusHandler;

- (instancetype)initWithDitto:(DITDitto *)ditto
       authenticationDelegate:(nullable id<DITAuthenticationDelegate>)authenticationDelegate;

- (void)loginWithTokenAndFeedback:(NSString *)token
                         provider:(nullable NSString *)provider
                       completion:(void (^)(NSString *__nullable, NSError *__nullable))completion;

- (void)authenticationStatusUpdated:(DITAuthenticationStatus *)authenticationStatus;
- (void)authenticationExpiring:(uint32_t)timeRemaining;
- (void)updateAndNotify:(BOOL)shouldNotify;
- (void)stopStatusObserver:(_DITAuthenticationStatusObserver *)authenticationStatusObserver;

@end

NS_ASSUME_NONNULL_END
