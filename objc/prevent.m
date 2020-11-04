#import <IOKit/pwr_mgt/IOPMLib.h>
 
// kIOPMAssertionTypeNoDisplaySleep prevents display sleep,
// kIOPMAssertionTypeNoIdleSleep prevents idle sleep
 
//reasonForActivity is a descriptive string used by the system whenever it needs
//  to tell the user why the system is not sleeping. For example,
//  "Mail Compacting Mailboxes" would be a useful string.

IOPMAssertionID inhibit(CFStringRef *reasonForActivity) {
    IOPMAssertionID assertionID;
    IOReturn success = IOPMAssertionCreateWithName(kIOPMAssertionTypeNoDisplaySleep,
                                    kIOPMAssertionLevelOn, reasonForActivity, &assertionID);
    return assertionID;
}

IOPMAssertionID uninhibit(IOPMAssertionID assertionID) {
    return IOPMAssertionRelease(assertionID);
}

