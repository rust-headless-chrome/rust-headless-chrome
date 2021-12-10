pub mod cdp {
    pub mod types {
        use serde::{Deserialize, Serialize};
        use std::fmt::Debug;
        pub type JsFloat = f64;
        pub type JsUInt = u32;
        pub type WindowId = JsUInt;
        pub type CallId = JsUInt;
        #[derive(Serialize, Debug)]
        pub struct MethodCall<T>
        where
            T: Debug,
        {
            #[serde(rename = "method")]
            method_name: &'static str,
            pub id: CallId,
            params: T,
        }
        impl<T> MethodCall<T>
        where
            T: Debug,
        {
            pub fn get_params(&self) -> &T {
                &self.params
            }
        }
        pub trait Method: Debug {
            const NAME: &'static str;
            type ReturnObject: serde::de::DeserializeOwned + std::fmt::Debug;
            fn to_method_call(self, call_id: CallId) -> MethodCall<Self>
            where
                Self: std::marker::Sized,
            {
                MethodCall {
                    id: call_id,
                    params: self,
                    method_name: Self::NAME,
                }
            }
        }
        #[derive(Deserialize, Debug, Clone, PartialEq)]
        #[serde(tag = "method")]
        #[allow(clippy::large_enum_variant)]
        pub enum Event {
            #[serde(rename = "Accessibility.loadComplete")]
            AccessibilityLoadComplete(super::Accessibility::events::LoadCompleteEvent),
            #[serde(rename = "Accessibility.nodesUpdated")]
            AccessibilityNodesUpdated(super::Accessibility::events::NodesUpdatedEvent),
            #[serde(rename = "Animation.animationCanceled")]
            AnimationCanceled(super::Animation::events::AnimationCanceledEvent),
            #[serde(rename = "Animation.animationCreated")]
            AnimationCreated(super::Animation::events::AnimationCreatedEvent),
            #[serde(rename = "Animation.animationStarted")]
            AnimationStarted(super::Animation::events::AnimationStartedEvent),
            #[serde(rename = "Audits.issueAdded")]
            AuditsIssueAdded(super::Audits::events::IssueAddedEvent),
            #[serde(rename = "BackgroundService.recordingStateChanged")]
            BackgroundServiceRecordingStateChanged(
                super::BackgroundService::events::RecordingStateChangedEvent,
            ),
            #[serde(rename = "BackgroundService.backgroundServiceEventReceived")]
            BackgroundServiceEventReceived(
                super::BackgroundService::events::BackgroundServiceEventReceivedEvent,
            ),
            #[serde(rename = "Browser.downloadWillBegin")]
            BrowserDownloadWillBegin(super::Browser::events::DownloadWillBeginEvent),
            #[serde(rename = "Browser.downloadProgress")]
            BrowserDownloadProgress(super::Browser::events::DownloadProgressEvent),
            #[serde(rename = "CSS.fontsUpdated")]
            CSSFontsUpdated(super::CSS::events::FontsUpdatedEvent),
            #[serde(rename = "CSS.mediaQueryResultChanged")]
            CSSMediaQueryResultChanged(super::CSS::events::MediaQueryResultChangedEvent),
            #[serde(rename = "CSS.styleSheetAdded")]
            CSSStyleSheetAdded(super::CSS::events::StyleSheetAddedEvent),
            #[serde(rename = "CSS.styleSheetChanged")]
            CSSStyleSheetChanged(super::CSS::events::StyleSheetChangedEvent),
            #[serde(rename = "CSS.styleSheetRemoved")]
            CSSStyleSheetRemoved(super::CSS::events::StyleSheetRemovedEvent),
            #[serde(rename = "Cast.sinksUpdated")]
            CastSinksUpdated(super::Cast::events::SinksUpdatedEvent),
            #[serde(rename = "Cast.issueUpdated")]
            CastIssueUpdated(super::Cast::events::IssueUpdatedEvent),
            #[serde(rename = "DOM.attributeModified")]
            DOMAttributeModified(super::DOM::events::AttributeModifiedEvent),
            #[serde(rename = "DOM.attributeRemoved")]
            DOMAttributeRemoved(super::DOM::events::AttributeRemovedEvent),
            #[serde(rename = "DOM.characterDataModified")]
            DOMCharacterDataModified(super::DOM::events::CharacterDataModifiedEvent),
            #[serde(rename = "DOM.childNodeCountUpdated")]
            DOMChildNodeCountUpdated(super::DOM::events::ChildNodeCountUpdatedEvent),
            #[serde(rename = "DOM.childNodeInserted")]
            DOMChildNodeInserted(super::DOM::events::ChildNodeInsertedEvent),
            #[serde(rename = "DOM.childNodeRemoved")]
            DOMChildNodeRemoved(super::DOM::events::ChildNodeRemovedEvent),
            #[serde(rename = "DOM.distributedNodesUpdated")]
            DOMDistributedNodesUpdated(super::DOM::events::DistributedNodesUpdatedEvent),
            #[serde(rename = "DOM.documentUpdated")]
            DOMDocumentUpdated(super::DOM::events::DocumentUpdatedEvent),
            #[serde(rename = "DOM.inlineStyleInvalidated")]
            DOMInlineStyleInvalidated(super::DOM::events::InlineStyleInvalidatedEvent),
            #[serde(rename = "DOM.pseudoElementAdded")]
            DOMPseudoElementAdded(super::DOM::events::PseudoElementAddedEvent),
            #[serde(rename = "DOM.pseudoElementRemoved")]
            DOMPseudoElementRemoved(super::DOM::events::PseudoElementRemovedEvent),
            #[serde(rename = "DOM.setChildNodes")]
            DOMSetChildNodes(super::DOM::events::SetChildNodesEvent),
            #[serde(rename = "DOM.shadowRootPopped")]
            DOMShadowRootPopped(super::DOM::events::ShadowRootPoppedEvent),
            #[serde(rename = "DOM.shadowRootPushed")]
            DOMShadowRootPushed(super::DOM::events::ShadowRootPushedEvent),
            #[serde(rename = "DOMStorage.domStorageItemAdded")]
            DOMStorageDomStorageItemAdded(super::DOMStorage::events::DomStorageItemAddedEvent),
            #[serde(rename = "DOMStorage.domStorageItemRemoved")]
            DOMStorageDomStorageItemRemoved(super::DOMStorage::events::DomStorageItemRemovedEvent),
            #[serde(rename = "DOMStorage.domStorageItemUpdated")]
            DOMStorageDomStorageItemUpdated(super::DOMStorage::events::DomStorageItemUpdatedEvent),
            #[serde(rename = "DOMStorage.domStorageItemsCleared")]
            DOMStorageDomStorageItemsCleared(
                super::DOMStorage::events::DomStorageItemsClearedEvent,
            ),
            #[serde(rename = "Database.addDatabase")]
            AddDatabase(super::Database::events::AddDatabaseEvent),
            #[serde(rename = "Emulation.virtualTimeBudgetExpired")]
            EmulationVirtualTimeBudgetExpired(
                super::Emulation::events::VirtualTimeBudgetExpiredEvent,
            ),
            #[serde(rename = "HeadlessExperimental.needsBeginFramesChanged")]
            HeadlessExperimentalNeedsBeginFramesChanged(
                super::HeadlessExperimental::events::NeedsBeginFramesChangedEvent,
            ),
            #[serde(rename = "Input.dragIntercepted")]
            InputDragIntercepted(super::Input::events::DragInterceptedEvent),
            #[serde(rename = "Inspector.detached")]
            InspectorDetached(super::Inspector::events::DetachedEvent),
            #[serde(rename = "Inspector.targetCrashed")]
            InspectorTargetCrashed(super::Inspector::events::TargetCrashedEvent),
            #[serde(rename = "Inspector.targetReloadedAfterCrash")]
            InspectorTargetReloadedAfterCrash(
                super::Inspector::events::TargetReloadedAfterCrashEvent,
            ),
            #[serde(rename = "LayerTree.layerPainted")]
            LayerTreeLayerPainted(super::LayerTree::events::LayerPaintedEvent),
            #[serde(rename = "LayerTree.layerTreeDidChange")]
            LayerTreeDidChange(super::LayerTree::events::LayerTreeDidChangeEvent),
            #[serde(rename = "Log.entryAdded")]
            LogEntryAdded(super::Log::events::EntryAddedEvent),
            #[serde(rename = "Network.dataReceived")]
            NetworkDataReceived(super::Network::events::DataReceivedEvent),
            #[serde(rename = "Network.eventSourceMessageReceived")]
            NetworkEventSourceMessageReceived(
                super::Network::events::EventSourceMessageReceivedEvent,
            ),
            #[serde(rename = "Network.loadingFailed")]
            NetworkLoadingFailed(super::Network::events::LoadingFailedEvent),
            #[serde(rename = "Network.loadingFinished")]
            NetworkLoadingFinished(super::Network::events::LoadingFinishedEvent),
            #[serde(rename = "Network.requestIntercepted")]
            NetworkRequestIntercepted(super::Network::events::RequestInterceptedEvent),
            #[serde(rename = "Network.requestServedFromCache")]
            NetworkRequestServedFromCache(super::Network::events::RequestServedFromCacheEvent),
            #[serde(rename = "Network.requestWillBeSent")]
            NetworkRequestWillBeSent(super::Network::events::RequestWillBeSentEvent),
            #[serde(rename = "Network.resourceChangedPriority")]
            NetworkResourceChangedPriority(super::Network::events::ResourceChangedPriorityEvent),
            #[serde(rename = "Network.signedExchangeReceived")]
            NetworkSignedExchangeReceived(super::Network::events::SignedExchangeReceivedEvent),
            #[serde(rename = "Network.responseReceived")]
            NetworkResponseReceived(super::Network::events::ResponseReceivedEvent),
            #[serde(rename = "Network.webSocketClosed")]
            NetworkWebSocketClosed(super::Network::events::WebSocketClosedEvent),
            #[serde(rename = "Network.webSocketCreated")]
            NetworkWebSocketCreated(super::Network::events::WebSocketCreatedEvent),
            #[serde(rename = "Network.webSocketFrameError")]
            NetworkWebSocketFrameError(super::Network::events::WebSocketFrameErrorEvent),
            #[serde(rename = "Network.webSocketFrameReceived")]
            NetworkWebSocketFrameReceived(super::Network::events::WebSocketFrameReceivedEvent),
            #[serde(rename = "Network.webSocketFrameSent")]
            NetworkWebSocketFrameSent(super::Network::events::WebSocketFrameSentEvent),
            #[serde(rename = "Network.webSocketHandshakeResponseReceived")]
            NetworkWebSocketHandshakeResponseReceived(
                super::Network::events::WebSocketHandshakeResponseReceivedEvent,
            ),
            #[serde(rename = "Network.webSocketWillSendHandshakeRequest")]
            NetworkWebSocketWillSendHandshakeRequest(
                super::Network::events::WebSocketWillSendHandshakeRequestEvent,
            ),
            #[serde(rename = "Network.webTransportCreated")]
            NetworkWebTransportCreated(super::Network::events::WebTransportCreatedEvent),
            #[serde(rename = "Network.webTransportConnectionEstablished")]
            NetworkWebTransportConnectionEstablished(
                super::Network::events::WebTransportConnectionEstablishedEvent,
            ),
            #[serde(rename = "Network.webTransportClosed")]
            NetworkWebTransportClosed(super::Network::events::WebTransportClosedEvent),
            #[serde(rename = "Network.requestWillBeSentExtraInfo")]
            NetworkRequestWillBeSentExtraInfo(
                super::Network::events::RequestWillBeSentExtraInfoEvent,
            ),
            #[serde(rename = "Network.responseReceivedExtraInfo")]
            NetworkResponseReceivedExtraInfo(
                super::Network::events::ResponseReceivedExtraInfoEvent,
            ),
            #[serde(rename = "Network.trustTokenOperationDone")]
            NetworkTrustTokenOperationDone(super::Network::events::TrustTokenOperationDoneEvent),
            #[serde(rename = "Network.subresourceWebBundleMetadataReceived")]
            NetworkSubresourceWebBundleMetadataReceived(
                super::Network::events::SubresourceWebBundleMetadataReceivedEvent,
            ),
            #[serde(rename = "Network.subresourceWebBundleMetadataError")]
            NetworkSubresourceWebBundleMetadataError(
                super::Network::events::SubresourceWebBundleMetadataErrorEvent,
            ),
            #[serde(rename = "Network.subresourceWebBundleInnerResponseParsed")]
            NetworkSubresourceWebBundleInnerResponseParsed(
                super::Network::events::SubresourceWebBundleInnerResponseParsedEvent,
            ),
            #[serde(rename = "Network.subresourceWebBundleInnerResponseError")]
            NetworkSubresourceWebBundleInnerResponseError(
                super::Network::events::SubresourceWebBundleInnerResponseErrorEvent,
            ),
            #[serde(rename = "Network.reportingApiReportAdded")]
            NetworkReportingApiReportAdded(super::Network::events::ReportingApiReportAddedEvent),
            #[serde(rename = "Network.reportingApiReportUpdated")]
            NetworkReportingApiReportUpdated(
                super::Network::events::ReportingApiReportUpdatedEvent,
            ),
            #[serde(rename = "Network.reportingApiEndpointsChangedForOrigin")]
            NetworkReportingApiEndpointsChangedForOrigin(
                super::Network::events::ReportingApiEndpointsChangedForOriginEvent,
            ),
            #[serde(rename = "Overlay.inspectNodeRequested")]
            OverlayInspectNodeRequested(super::Overlay::events::InspectNodeRequestedEvent),
            #[serde(rename = "Overlay.nodeHighlightRequested")]
            OverlayNodeHighlightRequested(super::Overlay::events::NodeHighlightRequestedEvent),
            #[serde(rename = "Overlay.screenshotRequested")]
            OverlayScreenshotRequested(super::Overlay::events::ScreenshotRequestedEvent),
            #[serde(rename = "Overlay.inspectModeCanceled")]
            OverlayInspectModeCanceled(super::Overlay::events::InspectModeCanceledEvent),
            #[serde(rename = "Page.domContentEventFired")]
            PageDomContentEventFired(super::Page::events::DomContentEventFiredEvent),
            #[serde(rename = "Page.fileChooserOpened")]
            PageFileChooserOpened(super::Page::events::FileChooserOpenedEvent),
            #[serde(rename = "Page.frameAttached")]
            PageFrameAttached(super::Page::events::FrameAttachedEvent),
            #[serde(rename = "Page.frameClearedScheduledNavigation")]
            PageFrameClearedScheduledNavigation(
                super::Page::events::FrameClearedScheduledNavigationEvent,
            ),
            #[serde(rename = "Page.frameDetached")]
            PageFrameDetached(super::Page::events::FrameDetachedEvent),
            #[serde(rename = "Page.frameNavigated")]
            PageFrameNavigated(super::Page::events::FrameNavigatedEvent),
            #[serde(rename = "Page.documentOpened")]
            PageDocumentOpened(super::Page::events::DocumentOpenedEvent),
            #[serde(rename = "Page.frameResized")]
            PageFrameResized(super::Page::events::FrameResizedEvent),
            #[serde(rename = "Page.frameRequestedNavigation")]
            PageFrameRequestedNavigation(super::Page::events::FrameRequestedNavigationEvent),
            #[serde(rename = "Page.frameScheduledNavigation")]
            PageFrameScheduledNavigation(super::Page::events::FrameScheduledNavigationEvent),
            #[serde(rename = "Page.frameStartedLoading")]
            PageFrameStartedLoading(super::Page::events::FrameStartedLoadingEvent),
            #[serde(rename = "Page.frameStoppedLoading")]
            PageFrameStoppedLoading(super::Page::events::FrameStoppedLoadingEvent),
            #[serde(rename = "Page.downloadWillBegin")]
            PageDownloadWillBegin(super::Page::events::DownloadWillBeginEvent),
            #[serde(rename = "Page.downloadProgress")]
            PageDownloadProgress(super::Page::events::DownloadProgressEvent),
            #[serde(rename = "Page.interstitialHidden")]
            PageInterstitialHidden(super::Page::events::InterstitialHiddenEvent),
            #[serde(rename = "Page.interstitialShown")]
            PageInterstitialShown(super::Page::events::InterstitialShownEvent),
            #[serde(rename = "Page.javascriptDialogClosed")]
            PageJavascriptDialogClosed(super::Page::events::JavascriptDialogClosedEvent),
            #[serde(rename = "Page.javascriptDialogOpening")]
            PageJavascriptDialogOpening(super::Page::events::JavascriptDialogOpeningEvent),
            #[serde(rename = "Page.lifecycleEvent")]
            PageLifecycleEvent(super::Page::events::LifecycleEventEvent),
            #[serde(rename = "Page.backForwardCacheNotUsed")]
            PageBackForwardCacheNotUsed(super::Page::events::BackForwardCacheNotUsedEvent),
            #[serde(rename = "Page.loadEventFired")]
            PageLoadEventFired(super::Page::events::LoadEventFiredEvent),
            #[serde(rename = "Page.navigatedWithinDocument")]
            PageNavigatedWithinDocument(super::Page::events::NavigatedWithinDocumentEvent),
            #[serde(rename = "Page.screencastFrame")]
            PageScreencastFrame(super::Page::events::ScreencastFrameEvent),
            #[serde(rename = "Page.screencastVisibilityChanged")]
            PageScreencastVisibilityChanged(super::Page::events::ScreencastVisibilityChangedEvent),
            #[serde(rename = "Page.windowOpen")]
            PageWindowOpen(super::Page::events::WindowOpenEvent),
            #[serde(rename = "Page.compilationCacheProduced")]
            PageCompilationCacheProduced(super::Page::events::CompilationCacheProducedEvent),
            #[serde(rename = "Performance.metrics")]
            PerformanceMetrics(super::Performance::events::MetricsEvent),
            #[serde(rename = "PerformanceTimeline.timelineEventAdded")]
            PerformanceTimelineTimelineEventAdded(
                super::PerformanceTimeline::events::TimelineEventAddedEvent,
            ),
            #[serde(rename = "Security.certificateError")]
            SecurityCertificateError(super::Security::events::CertificateErrorEvent),
            #[serde(rename = "Security.visibleSecurityStateChanged")]
            VisibleSecurityStateChanged(super::Security::events::VisibleSecurityStateChangedEvent),
            #[serde(rename = "Security.securityStateChanged")]
            SecurityStateChanged(super::Security::events::SecurityStateChangedEvent),
            #[serde(rename = "ServiceWorker.workerErrorReported")]
            ServiceWorkerWorkerErrorReported(
                super::ServiceWorker::events::WorkerErrorReportedEvent,
            ),
            #[serde(rename = "ServiceWorker.workerRegistrationUpdated")]
            ServiceWorkerWorkerRegistrationUpdated(
                super::ServiceWorker::events::WorkerRegistrationUpdatedEvent,
            ),
            #[serde(rename = "ServiceWorker.workerVersionUpdated")]
            ServiceWorkerWorkerVersionUpdated(
                super::ServiceWorker::events::WorkerVersionUpdatedEvent,
            ),
            #[serde(rename = "Storage.cacheStorageContentUpdated")]
            CacheStorageContentUpdated(super::Storage::events::CacheStorageContentUpdatedEvent),
            #[serde(rename = "Storage.cacheStorageListUpdated")]
            CacheStorageListUpdated(super::Storage::events::CacheStorageListUpdatedEvent),
            #[serde(rename = "Storage.indexedDBContentUpdated")]
            StorageIndexedDBContentUpdated(super::Storage::events::IndexedDBContentUpdatedEvent),
            #[serde(rename = "Storage.indexedDBListUpdated")]
            StorageIndexedDBListUpdated(super::Storage::events::IndexedDBListUpdatedEvent),
            #[serde(rename = "Target.attachedToTarget")]
            AttachedToTarget(super::Target::events::AttachedToTargetEvent),
            #[serde(rename = "Target.detachedFromTarget")]
            DetachedFromTarget(super::Target::events::DetachedFromTargetEvent),
            #[serde(rename = "Target.receivedMessageFromTarget")]
            ReceivedMessageFromTarget(super::Target::events::ReceivedMessageFromTargetEvent),
            #[serde(rename = "Target.targetCreated")]
            TargetCreated(super::Target::events::TargetCreatedEvent),
            #[serde(rename = "Target.targetDestroyed")]
            TargetDestroyed(super::Target::events::TargetDestroyedEvent),
            #[serde(rename = "Target.targetCrashed")]
            TargetCrashed(super::Target::events::TargetCrashedEvent),
            #[serde(rename = "Target.targetInfoChanged")]
            TargetInfoChanged(super::Target::events::TargetInfoChangedEvent),
            #[serde(rename = "Tethering.accepted")]
            TetheringAccepted(super::Tethering::events::AcceptedEvent),
            #[serde(rename = "Tracing.bufferUsage")]
            TracingBufferUsage(super::Tracing::events::BufferUsageEvent),
            #[serde(rename = "Tracing.dataCollected")]
            TracingDataCollected(super::Tracing::events::DataCollectedEvent),
            #[serde(rename = "Tracing.tracingComplete")]
            TracingComplete(super::Tracing::events::TracingCompleteEvent),
            #[serde(rename = "Fetch.requestPaused")]
            FetchRequestPaused(super::Fetch::events::RequestPausedEvent),
            #[serde(rename = "Fetch.authRequired")]
            FetchAuthRequired(super::Fetch::events::AuthRequiredEvent),
            #[serde(rename = "WebAudio.contextCreated")]
            WebAudioContextCreated(super::WebAudio::events::ContextCreatedEvent),
            #[serde(rename = "WebAudio.contextWillBeDestroyed")]
            WebAudioContextWillBeDestroyed(super::WebAudio::events::ContextWillBeDestroyedEvent),
            #[serde(rename = "WebAudio.contextChanged")]
            WebAudioContextChanged(super::WebAudio::events::ContextChangedEvent),
            #[serde(rename = "WebAudio.audioListenerCreated")]
            WebAudioAudioListenerCreated(super::WebAudio::events::AudioListenerCreatedEvent),
            #[serde(rename = "WebAudio.audioListenerWillBeDestroyed")]
            WebAudioAudioListenerWillBeDestroyed(
                super::WebAudio::events::AudioListenerWillBeDestroyedEvent,
            ),
            #[serde(rename = "WebAudio.audioNodeCreated")]
            WebAudioAudioNodeCreated(super::WebAudio::events::AudioNodeCreatedEvent),
            #[serde(rename = "WebAudio.audioNodeWillBeDestroyed")]
            WebAudioAudioNodeWillBeDestroyed(
                super::WebAudio::events::AudioNodeWillBeDestroyedEvent,
            ),
            #[serde(rename = "WebAudio.audioParamCreated")]
            WebAudioAudioParamCreated(super::WebAudio::events::AudioParamCreatedEvent),
            #[serde(rename = "WebAudio.audioParamWillBeDestroyed")]
            WebAudioAudioParamWillBeDestroyed(
                super::WebAudio::events::AudioParamWillBeDestroyedEvent,
            ),
            #[serde(rename = "WebAudio.nodesConnected")]
            WebAudioNodesConnected(super::WebAudio::events::NodesConnectedEvent),
            #[serde(rename = "WebAudio.nodesDisconnected")]
            WebAudioNodesDisconnected(super::WebAudio::events::NodesDisconnectedEvent),
            #[serde(rename = "WebAudio.nodeParamConnected")]
            WebAudioNodeParamConnected(super::WebAudio::events::NodeParamConnectedEvent),
            #[serde(rename = "WebAudio.nodeParamDisconnected")]
            WebAudioNodeParamDisconnected(super::WebAudio::events::NodeParamDisconnectedEvent),
            #[serde(rename = "Media.playerPropertiesChanged")]
            MediaPlayerPropertiesChanged(super::Media::events::PlayerPropertiesChangedEvent),
            #[serde(rename = "Media.playerEventsAdded")]
            MediaPlayerEventsAdded(super::Media::events::PlayerEventsAddedEvent),
            #[serde(rename = "Media.playerMessagesLogged")]
            MediaPlayerMessagesLogged(super::Media::events::PlayerMessagesLoggedEvent),
            #[serde(rename = "Media.playerErrorsRaised")]
            MediaPlayerErrorsRaised(super::Media::events::PlayerErrorsRaisedEvent),
            #[serde(rename = "Media.playersCreated")]
            MediaPlayersCreated(super::Media::events::PlayersCreatedEvent),
            #[serde(rename = "Console.messageAdded")]
            ConsoleMessageAdded(super::Console::events::MessageAddedEvent),
            #[serde(rename = "Debugger.breakpointResolved")]
            DebuggerBreakpointResolved(super::Debugger::events::BreakpointResolvedEvent),
            #[serde(rename = "Debugger.paused")]
            DebuggerPaused(super::Debugger::events::PausedEvent),
            #[serde(rename = "Debugger.resumed")]
            DebuggerResumed(super::Debugger::events::ResumedEvent),
            #[serde(rename = "Debugger.scriptFailedToParse")]
            DebuggerScriptFailedToParse(super::Debugger::events::ScriptFailedToParseEvent),
            #[serde(rename = "Debugger.scriptParsed")]
            DebuggerScriptParsed(super::Debugger::events::ScriptParsedEvent),
            #[serde(rename = "HeapProfiler.addHeapSnapshotChunk")]
            HeapProfilerAddHeapSnapshotChunk(
                super::HeapProfiler::events::AddHeapSnapshotChunkEvent,
            ),
            #[serde(rename = "HeapProfiler.heapStatsUpdate")]
            HeapProfilerHeapStatsUpdate(super::HeapProfiler::events::HeapStatsUpdateEvent),
            #[serde(rename = "HeapProfiler.lastSeenObjectId")]
            HeapProfilerLastSeenObjectId(super::HeapProfiler::events::LastSeenObjectIdEvent),
            #[serde(rename = "HeapProfiler.reportHeapSnapshotProgress")]
            HeapProfilerReportHeapSnapshotProgress(
                super::HeapProfiler::events::ReportHeapSnapshotProgressEvent,
            ),
            #[serde(rename = "HeapProfiler.resetProfiles")]
            HeapProfilerResetProfiles(super::HeapProfiler::events::ResetProfilesEvent),
            #[serde(rename = "Profiler.consoleProfileFinished")]
            ProfilerConsoleProfileFinished(super::Profiler::events::ConsoleProfileFinishedEvent),
            #[serde(rename = "Profiler.consoleProfileStarted")]
            ProfilerConsoleProfileStarted(super::Profiler::events::ConsoleProfileStartedEvent),
            #[serde(rename = "Profiler.preciseCoverageDeltaUpdate")]
            ProfilerPreciseCoverageDeltaUpdate(
                super::Profiler::events::PreciseCoverageDeltaUpdateEvent,
            ),
            #[serde(rename = "Runtime.bindingCalled")]
            RuntimeBindingCalled(super::Runtime::events::BindingCalledEvent),
            #[serde(rename = "Runtime.consoleAPICalled")]
            RuntimeConsoleAPICalled(super::Runtime::events::ConsoleAPICalledEvent),
            #[serde(rename = "Runtime.exceptionRevoked")]
            RuntimeExceptionRevoked(super::Runtime::events::ExceptionRevokedEvent),
            #[serde(rename = "Runtime.exceptionThrown")]
            RuntimeExceptionThrown(super::Runtime::events::ExceptionThrownEvent),
            #[serde(rename = "Runtime.executionContextCreated")]
            RuntimeExecutionContextCreated(super::Runtime::events::ExecutionContextCreatedEvent),
            #[serde(rename = "Runtime.executionContextDestroyed")]
            RuntimeExecutionContextDestroyed(
                super::Runtime::events::ExecutionContextDestroyedEvent,
            ),
            #[serde(rename = "Runtime.executionContextsCleared")]
            RuntimeExecutionContextsCleared(super::Runtime::events::ExecutionContextsClearedEvent),
            #[serde(rename = "Runtime.inspectRequested")]
            RuntimeInspectRequested(super::Runtime::events::InspectRequestedEvent),
        }
    }
    pub mod Console {
        use super::types::*;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum ConsoleMessageSource {
            Xml,
            Javascript,
            Network,
            ConsoleApi,
            Storage,
            Appcache,
            Rendering,
            Security,
            Other,
            Deprecation,
            Worker,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ConsoleMessageLevel {
            Log,
            Warning,
            Error,
            Debug,
            Info,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ConsoleMessage {
            pub source: ConsoleMessageSource,
            pub level: ConsoleMessageLevel,
            #[serde(default)]
            pub text: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub line: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub column: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearMessages(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearMessagesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        impl Method for ClearMessages {
            const NAME: &'static str = "Console.clearMessages";
            type ReturnObject = ClearMessagesReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Console.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Console.enable";
            type ReturnObject = EnableReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct MessageAddedEvent {
                pub params: MessageAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct MessageAddedEventParams {
                pub message: super::ConsoleMessage,
            }
        }
    }
    pub mod Debugger {
        use super::types::*;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type BreakpointId = String;
        pub type CallFrameId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum ScopeType {
            Global,
            Local,
            With,
            Closure,
            Catch,
            Block,
            Script,
            Eval,
            Module,
            WasmExpressionStack,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum BreakLocationType {
            DebuggerStatement,
            Call,
            Return,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum ScriptLanguage {
            JavaScript,
            WebAssembly,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum DebugSymbolsType {
            None,
            SourceMap,
            EmbeddedDwarf,
            ExternalDwarf,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ContinueToLocationTarget_call_framesOption {
            Any,
            Current,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum SetInstrumentationBreakpointInstrumentationOption {
            BeforeScriptExecution,
            BeforeScriptWithSourceMapExecution,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum SetPauseOnExceptionsStateOption {
            None,
            Uncaught,
            All,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum PausedEventReasonOption {
            Ambiguous,
            Assert,
            CspViolation,
            DebugCommand,
            Dom,
            EventListener,
            Exception,
            Instrumentation,
            Oom,
            Other,
            PromiseRejection,
            Xhr,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Location {
            pub script_id: Runtime::ScriptId,
            #[serde(default)]
            pub line_number: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub column_number: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ScriptPosition {
            #[serde(default)]
            pub line_number: JsUInt,
            #[serde(default)]
            pub column_number: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LocationRange {
            pub script_id: Runtime::ScriptId,
            pub start: ScriptPosition,
            pub end: ScriptPosition,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CallFrame {
            pub call_frame_id: CallFrameId,
            #[serde(default)]
            pub function_name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub function_location: Option<Location>,
            pub location: Location,
            #[serde(default)]
            pub url: String,
            pub scope_chain: Vec<Scope>,
            pub this: Runtime::RemoteObject,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub return_value: Option<Runtime::RemoteObject>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Scope {
            pub Type: ScopeType,
            pub object: Runtime::RemoteObject,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub start_location: Option<Location>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub end_location: Option<Location>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SearchMatch {
            #[serde(default)]
            pub line_number: JsFloat,
            #[serde(default)]
            pub line_content: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BreakLocation {
            pub script_id: Runtime::ScriptId,
            #[serde(default)]
            pub line_number: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub column_number: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub Type: Option<BreakLocationType>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DebugSymbols {
            pub Type: DebugSymbolsType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub external_url: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContinueToLocation {
            pub location: Location,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub target_call_frames: Option<ContinueToLocationTarget_call_framesOption>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub max_scripts_cache_size: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EvaluateOnCallFrame {
            pub call_frame_id: CallFrameId,
            #[serde(default)]
            pub expression: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub object_group: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub include_command_line_api: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub silent: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub return_by_value: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub generate_preview: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub throw_on_side_effect: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub timeout: Option<Runtime::TimeDelta>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetPossibleBreakpoints {
            pub start: Location,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub end: Option<Location>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub restrict_to_function: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetScriptSource {
            pub script_id: Runtime::ScriptId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetWasmBytecode {
            pub script_id: Runtime::ScriptId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetStackTrace {
            pub stack_trace_id: Runtime::StackTraceId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Pause(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PauseOnAsyncCall {
            pub parent_stack_trace_id: Runtime::StackTraceId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveBreakpoint {
            pub breakpoint_id: BreakpointId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RestartFrame {
            pub call_frame_id: CallFrameId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Resume {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub terminate_on_resume: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SearchInContent {
            pub script_id: Runtime::ScriptId,
            #[serde(default)]
            pub query: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub case_sensitive: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub is_regex: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAsyncCallStackDepth {
            #[serde(default)]
            pub max_depth: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBlackboxPatterns {
            #[serde(default)]
            pub patterns: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBlackboxedRanges {
            pub script_id: Runtime::ScriptId,
            pub positions: Vec<ScriptPosition>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBreakpoint {
            pub location: Location,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub condition: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInstrumentationBreakpoint {
            pub instrumentation: SetInstrumentationBreakpointInstrumentationOption,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBreakpointByUrl {
            #[serde(default)]
            pub line_number: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url_regex: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub script_hash: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub column_number: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub condition: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBreakpointOnFunctionCall {
            pub object_id: Runtime::RemoteObjectId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub condition: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBreakpointsActive {
            #[serde(default)]
            pub active: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPauseOnExceptions {
            pub state: SetPauseOnExceptionsStateOption,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetReturnValue {
            pub new_value: Runtime::CallArgument,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetScriptSource {
            pub script_id: Runtime::ScriptId,
            #[serde(default)]
            pub script_source: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub dry_run: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetSkipAllPauses {
            #[serde(default)]
            pub skip: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetVariableValue {
            #[serde(default)]
            pub scope_number: JsUInt,
            #[serde(default)]
            pub variable_name: String,
            pub new_value: Runtime::CallArgument,
            pub call_frame_id: CallFrameId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StepInto {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub break_on_async_call: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub skip_list: Option<Vec<LocationRange>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StepOut(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StepOver {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub skip_list: Option<Vec<LocationRange>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContinueToLocationReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {
            pub debugger_id: Runtime::UniqueDebuggerId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EvaluateOnCallFrameReturnObject {
            pub result: Runtime::RemoteObject,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub exception_details: Option<Runtime::ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetPossibleBreakpointsReturnObject {
            pub locations: Vec<BreakLocation>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetScriptSourceReturnObject {
            #[serde(default)]
            pub script_source: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub bytecode: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetWasmBytecodeReturnObject {
            #[serde(default)]
            pub bytecode: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetStackTraceReturnObject {
            pub stack_trace: Runtime::StackTrace,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PauseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PauseOnAsyncCallReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RestartFrameReturnObject {
            pub call_frames: Vec<CallFrame>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub async_stack_trace: Option<Runtime::StackTrace>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub async_stack_trace_id: Option<Runtime::StackTraceId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResumeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SearchInContentReturnObject {
            pub result: Vec<SearchMatch>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAsyncCallStackDepthReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBlackboxPatternsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBlackboxedRangesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBreakpointReturnObject {
            pub breakpoint_id: BreakpointId,
            pub actual_location: Location,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInstrumentationBreakpointReturnObject {
            pub breakpoint_id: BreakpointId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBreakpointByUrlReturnObject {
            pub breakpoint_id: BreakpointId,
            pub locations: Vec<Location>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBreakpointOnFunctionCallReturnObject {
            pub breakpoint_id: BreakpointId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBreakpointsActiveReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPauseOnExceptionsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetReturnValueReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetScriptSourceReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub call_frames: Option<Vec<CallFrame>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub stack_changed: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub async_stack_trace: Option<Runtime::StackTrace>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub async_stack_trace_id: Option<Runtime::StackTraceId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub exception_details: Option<Runtime::ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetSkipAllPausesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetVariableValueReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StepIntoReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StepOutReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StepOverReturnObject {}
        impl Method for ContinueToLocation {
            const NAME: &'static str = "Debugger.continueToLocation";
            type ReturnObject = ContinueToLocationReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Debugger.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Debugger.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for EvaluateOnCallFrame {
            const NAME: &'static str = "Debugger.evaluateOnCallFrame";
            type ReturnObject = EvaluateOnCallFrameReturnObject;
        }
        impl Method for GetPossibleBreakpoints {
            const NAME: &'static str = "Debugger.getPossibleBreakpoints";
            type ReturnObject = GetPossibleBreakpointsReturnObject;
        }
        impl Method for GetScriptSource {
            const NAME: &'static str = "Debugger.getScriptSource";
            type ReturnObject = GetScriptSourceReturnObject;
        }
        impl Method for GetWasmBytecode {
            const NAME: &'static str = "Debugger.getWasmBytecode";
            type ReturnObject = GetWasmBytecodeReturnObject;
        }
        impl Method for GetStackTrace {
            const NAME: &'static str = "Debugger.getStackTrace";
            type ReturnObject = GetStackTraceReturnObject;
        }
        impl Method for Pause {
            const NAME: &'static str = "Debugger.pause";
            type ReturnObject = PauseReturnObject;
        }
        impl Method for PauseOnAsyncCall {
            const NAME: &'static str = "Debugger.pauseOnAsyncCall";
            type ReturnObject = PauseOnAsyncCallReturnObject;
        }
        impl Method for RemoveBreakpoint {
            const NAME: &'static str = "Debugger.removeBreakpoint";
            type ReturnObject = RemoveBreakpointReturnObject;
        }
        impl Method for RestartFrame {
            const NAME: &'static str = "Debugger.restartFrame";
            type ReturnObject = RestartFrameReturnObject;
        }
        impl Method for Resume {
            const NAME: &'static str = "Debugger.resume";
            type ReturnObject = ResumeReturnObject;
        }
        impl Method for SearchInContent {
            const NAME: &'static str = "Debugger.searchInContent";
            type ReturnObject = SearchInContentReturnObject;
        }
        impl Method for SetAsyncCallStackDepth {
            const NAME: &'static str = "Debugger.setAsyncCallStackDepth";
            type ReturnObject = SetAsyncCallStackDepthReturnObject;
        }
        impl Method for SetBlackboxPatterns {
            const NAME: &'static str = "Debugger.setBlackboxPatterns";
            type ReturnObject = SetBlackboxPatternsReturnObject;
        }
        impl Method for SetBlackboxedRanges {
            const NAME: &'static str = "Debugger.setBlackboxedRanges";
            type ReturnObject = SetBlackboxedRangesReturnObject;
        }
        impl Method for SetBreakpoint {
            const NAME: &'static str = "Debugger.setBreakpoint";
            type ReturnObject = SetBreakpointReturnObject;
        }
        impl Method for SetInstrumentationBreakpoint {
            const NAME: &'static str = "Debugger.setInstrumentationBreakpoint";
            type ReturnObject = SetInstrumentationBreakpointReturnObject;
        }
        impl Method for SetBreakpointByUrl {
            const NAME: &'static str = "Debugger.setBreakpointByUrl";
            type ReturnObject = SetBreakpointByUrlReturnObject;
        }
        impl Method for SetBreakpointOnFunctionCall {
            const NAME: &'static str = "Debugger.setBreakpointOnFunctionCall";
            type ReturnObject = SetBreakpointOnFunctionCallReturnObject;
        }
        impl Method for SetBreakpointsActive {
            const NAME: &'static str = "Debugger.setBreakpointsActive";
            type ReturnObject = SetBreakpointsActiveReturnObject;
        }
        impl Method for SetPauseOnExceptions {
            const NAME: &'static str = "Debugger.setPauseOnExceptions";
            type ReturnObject = SetPauseOnExceptionsReturnObject;
        }
        impl Method for SetReturnValue {
            const NAME: &'static str = "Debugger.setReturnValue";
            type ReturnObject = SetReturnValueReturnObject;
        }
        impl Method for SetScriptSource {
            const NAME: &'static str = "Debugger.setScriptSource";
            type ReturnObject = SetScriptSourceReturnObject;
        }
        impl Method for SetSkipAllPauses {
            const NAME: &'static str = "Debugger.setSkipAllPauses";
            type ReturnObject = SetSkipAllPausesReturnObject;
        }
        impl Method for SetVariableValue {
            const NAME: &'static str = "Debugger.setVariableValue";
            type ReturnObject = SetVariableValueReturnObject;
        }
        impl Method for StepInto {
            const NAME: &'static str = "Debugger.stepInto";
            type ReturnObject = StepIntoReturnObject;
        }
        impl Method for StepOut {
            const NAME: &'static str = "Debugger.stepOut";
            type ReturnObject = StepOutReturnObject;
        }
        impl Method for StepOver {
            const NAME: &'static str = "Debugger.stepOver";
            type ReturnObject = StepOverReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct BreakpointResolvedEvent {
                pub params: BreakpointResolvedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct BreakpointResolvedEventParams {
                pub breakpoint_id: super::BreakpointId,
                pub location: super::Location,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PausedEvent {
                pub params: PausedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct PausedEventParams {
                pub call_frames: Vec<super::CallFrame>,
                pub reason: super::PausedEventReasonOption,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub hit_breakpoints: Option<Vec<String>>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub async_stack_trace: Option<super::super::Runtime::StackTrace>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub async_stack_trace_id: Option<super::super::Runtime::StackTraceId>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub async_call_stack_trace_id: Option<super::super::Runtime::StackTraceId>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ResumedEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ScriptFailedToParseEvent {
                pub params: ScriptFailedToParseEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ScriptFailedToParseEventParams {
                pub script_id: super::super::Runtime::ScriptId,
                #[serde(default)]
                pub url: String,
                #[serde(default)]
                pub start_line: JsUInt,
                #[serde(default)]
                pub start_column: JsUInt,
                #[serde(default)]
                pub end_line: JsUInt,
                #[serde(default)]
                pub end_column: JsUInt,
                pub execution_context_id: super::super::Runtime::ExecutionContextId,
                #[serde(default)]
                pub hash: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub source_map_url: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub has_source_url: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub is_module: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub length: Option<JsUInt>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub stack_trace: Option<super::super::Runtime::StackTrace>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub code_offset: Option<JsUInt>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub script_language: Option<super::super::Debugger::ScriptLanguage>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub embedder_name: Option<String>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ScriptParsedEvent {
                pub params: ScriptParsedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ScriptParsedEventParams {
                pub script_id: super::super::Runtime::ScriptId,
                #[serde(default)]
                pub url: String,
                #[serde(default)]
                pub start_line: JsUInt,
                #[serde(default)]
                pub start_column: JsUInt,
                #[serde(default)]
                pub end_line: JsUInt,
                #[serde(default)]
                pub end_column: JsUInt,
                pub execution_context_id: super::super::Runtime::ExecutionContextId,
                #[serde(default)]
                pub hash: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub is_live_edit: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub source_map_url: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub has_source_url: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub is_module: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub length: Option<JsUInt>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub stack_trace: Option<super::super::Runtime::StackTrace>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub code_offset: Option<JsUInt>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub script_language: Option<super::super::Debugger::ScriptLanguage>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub debug_symbols: Option<super::super::Debugger::DebugSymbols>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub embedder_name: Option<String>,
            }
        }
    }
    pub mod HeapProfiler {
        use super::types::*;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type HeapSnapshotObjectId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SamplingHeapProfileNode {
            pub call_frame: Runtime::CallFrame,
            #[serde(default)]
            pub self_size: JsFloat,
            #[serde(default)]
            pub id: JsUInt,
            pub children: Vec<SamplingHeapProfileNode>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SamplingHeapProfileSample {
            #[serde(default)]
            pub size: JsFloat,
            #[serde(default)]
            pub node_id: JsUInt,
            #[serde(default)]
            pub ordinal: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SamplingHeapProfile {
            pub head: SamplingHeapProfileNode,
            pub samples: Vec<SamplingHeapProfileSample>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddInspectedHeapObject {
            pub heap_object_id: HeapSnapshotObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CollectGarbage(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetHeapObjectId {
            pub object_id: Runtime::RemoteObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetObjectByHeapObjectId {
            pub object_id: HeapSnapshotObjectId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub object_group: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetSamplingProfile(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartSampling {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub sampling_interval: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartTrackingHeapObjects {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub track_allocations: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopSampling(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopTrackingHeapObjects {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub report_progress: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub treat_global_objects_as_roots: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub capture_numeric_value: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeHeapSnapshot {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub report_progress: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub treat_global_objects_as_roots: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub capture_numeric_value: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddInspectedHeapObjectReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CollectGarbageReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetHeapObjectIdReturnObject {
            pub heap_snapshot_object_id: HeapSnapshotObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetObjectByHeapObjectIdReturnObject {
            pub result: Runtime::RemoteObject,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetSamplingProfileReturnObject {
            pub profile: SamplingHeapProfile,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartSamplingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartTrackingHeapObjectsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopSamplingReturnObject {
            pub profile: SamplingHeapProfile,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopTrackingHeapObjectsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeHeapSnapshotReturnObject {}
        impl Method for AddInspectedHeapObject {
            const NAME: &'static str = "HeapProfiler.addInspectedHeapObject";
            type ReturnObject = AddInspectedHeapObjectReturnObject;
        }
        impl Method for CollectGarbage {
            const NAME: &'static str = "HeapProfiler.collectGarbage";
            type ReturnObject = CollectGarbageReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "HeapProfiler.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "HeapProfiler.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetHeapObjectId {
            const NAME: &'static str = "HeapProfiler.getHeapObjectId";
            type ReturnObject = GetHeapObjectIdReturnObject;
        }
        impl Method for GetObjectByHeapObjectId {
            const NAME: &'static str = "HeapProfiler.getObjectByHeapObjectId";
            type ReturnObject = GetObjectByHeapObjectIdReturnObject;
        }
        impl Method for GetSamplingProfile {
            const NAME: &'static str = "HeapProfiler.getSamplingProfile";
            type ReturnObject = GetSamplingProfileReturnObject;
        }
        impl Method for StartSampling {
            const NAME: &'static str = "HeapProfiler.startSampling";
            type ReturnObject = StartSamplingReturnObject;
        }
        impl Method for StartTrackingHeapObjects {
            const NAME: &'static str = "HeapProfiler.startTrackingHeapObjects";
            type ReturnObject = StartTrackingHeapObjectsReturnObject;
        }
        impl Method for StopSampling {
            const NAME: &'static str = "HeapProfiler.stopSampling";
            type ReturnObject = StopSamplingReturnObject;
        }
        impl Method for StopTrackingHeapObjects {
            const NAME: &'static str = "HeapProfiler.stopTrackingHeapObjects";
            type ReturnObject = StopTrackingHeapObjectsReturnObject;
        }
        impl Method for TakeHeapSnapshot {
            const NAME: &'static str = "HeapProfiler.takeHeapSnapshot";
            type ReturnObject = TakeHeapSnapshotReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AddHeapSnapshotChunkEvent {
                pub params: AddHeapSnapshotChunkEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AddHeapSnapshotChunkEventParams {
                #[serde(default)]
                pub chunk: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct HeapStatsUpdateEvent {
                pub params: HeapStatsUpdateEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct HeapStatsUpdateEventParams {
                #[serde(default)]
                pub stats_update: Vec<JsUInt>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LastSeenObjectIdEvent {
                pub params: LastSeenObjectIdEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct LastSeenObjectIdEventParams {
                #[serde(default)]
                pub last_seen_object_id: JsUInt,
                #[serde(default)]
                pub timestamp: JsFloat,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ReportHeapSnapshotProgressEvent {
                pub params: ReportHeapSnapshotProgressEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ReportHeapSnapshotProgressEventParams {
                #[serde(default)]
                pub done: JsUInt,
                #[serde(default)]
                pub total: JsUInt,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub finished: Option<bool>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ResetProfilesEvent(pub Option<serde_json::Value>);
        }
    }
    pub mod Profiler {
        use super::types::*;
        use super::Debugger;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ProfileNode {
            #[serde(default)]
            pub id: JsUInt,
            pub call_frame: Runtime::CallFrame,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub hit_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub children: Option<Vec<JsUInt>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub deopt_reason: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub position_ticks: Option<Vec<PositionTickInfo>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Profile {
            pub nodes: Vec<ProfileNode>,
            #[serde(default)]
            pub start_time: JsFloat,
            #[serde(default)]
            pub end_time: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub samples: Option<Vec<JsUInt>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub time_deltas: Option<Vec<JsUInt>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PositionTickInfo {
            #[serde(default)]
            pub line: JsUInt,
            #[serde(default)]
            pub ticks: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CoverageRange {
            #[serde(default)]
            pub start_offset: JsUInt,
            #[serde(default)]
            pub end_offset: JsUInt,
            #[serde(default)]
            pub count: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FunctionCoverage {
            #[serde(default)]
            pub function_name: String,
            pub ranges: Vec<CoverageRange>,
            #[serde(default)]
            pub is_block_coverage: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ScriptCoverage {
            pub script_id: Runtime::ScriptId,
            #[serde(default)]
            pub url: String,
            pub functions: Vec<FunctionCoverage>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TypeObject {
            #[serde(default)]
            pub name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TypeProfileEntry {
            #[serde(default)]
            pub offset: JsUInt,
            pub Types: Vec<TypeObject>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ScriptTypeProfile {
            pub script_id: Runtime::ScriptId,
            #[serde(default)]
            pub url: String,
            pub entries: Vec<TypeProfileEntry>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBestEffortCoverage(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetSamplingInterval {
            #[serde(default)]
            pub interval: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Start(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartPreciseCoverage {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub call_count: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub detailed: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub allow_triggered_updates: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartTypeProfile(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Stop(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopPreciseCoverage(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopTypeProfile(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakePreciseCoverage(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeTypeProfile(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBestEffortCoverageReturnObject {
            pub result: Vec<ScriptCoverage>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetSamplingIntervalReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartPreciseCoverageReturnObject {
            #[serde(default)]
            pub timestamp: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartTypeProfileReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopReturnObject {
            pub profile: Profile,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopPreciseCoverageReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopTypeProfileReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakePreciseCoverageReturnObject {
            pub result: Vec<ScriptCoverage>,
            #[serde(default)]
            pub timestamp: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeTypeProfileReturnObject {
            pub result: Vec<ScriptTypeProfile>,
        }
        impl Method for Disable {
            const NAME: &'static str = "Profiler.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Profiler.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetBestEffortCoverage {
            const NAME: &'static str = "Profiler.getBestEffortCoverage";
            type ReturnObject = GetBestEffortCoverageReturnObject;
        }
        impl Method for SetSamplingInterval {
            const NAME: &'static str = "Profiler.setSamplingInterval";
            type ReturnObject = SetSamplingIntervalReturnObject;
        }
        impl Method for Start {
            const NAME: &'static str = "Profiler.start";
            type ReturnObject = StartReturnObject;
        }
        impl Method for StartPreciseCoverage {
            const NAME: &'static str = "Profiler.startPreciseCoverage";
            type ReturnObject = StartPreciseCoverageReturnObject;
        }
        impl Method for StartTypeProfile {
            const NAME: &'static str = "Profiler.startTypeProfile";
            type ReturnObject = StartTypeProfileReturnObject;
        }
        impl Method for Stop {
            const NAME: &'static str = "Profiler.stop";
            type ReturnObject = StopReturnObject;
        }
        impl Method for StopPreciseCoverage {
            const NAME: &'static str = "Profiler.stopPreciseCoverage";
            type ReturnObject = StopPreciseCoverageReturnObject;
        }
        impl Method for StopTypeProfile {
            const NAME: &'static str = "Profiler.stopTypeProfile";
            type ReturnObject = StopTypeProfileReturnObject;
        }
        impl Method for TakePreciseCoverage {
            const NAME: &'static str = "Profiler.takePreciseCoverage";
            type ReturnObject = TakePreciseCoverageReturnObject;
        }
        impl Method for TakeTypeProfile {
            const NAME: &'static str = "Profiler.takeTypeProfile";
            type ReturnObject = TakeTypeProfileReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ConsoleProfileFinishedEvent {
                pub params: ConsoleProfileFinishedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ConsoleProfileFinishedEventParams {
                #[serde(default)]
                pub id: String,
                pub location: super::super::Debugger::Location,
                pub profile: super::Profile,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub title: Option<String>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ConsoleProfileStartedEvent {
                pub params: ConsoleProfileStartedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ConsoleProfileStartedEventParams {
                #[serde(default)]
                pub id: String,
                pub location: super::super::Debugger::Location,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub title: Option<String>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PreciseCoverageDeltaUpdateEvent {
                pub params: PreciseCoverageDeltaUpdateEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct PreciseCoverageDeltaUpdateEventParams {
                #[serde(default)]
                pub timestamp: JsFloat,
                #[serde(default)]
                pub occasion: String,
                pub result: Vec<super::ScriptCoverage>,
            }
        }
    }
    pub mod Runtime {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type ScriptId = String;
        pub type RemoteObjectId = String;
        pub type UnserializableValue = String;
        pub type ExecutionContextId = JsUInt;
        pub type Timestamp = JsFloat;
        pub type TimeDelta = JsFloat;
        pub type UniqueDebuggerId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum RemoteObjectType {
            Object,
            Function,
            Undefined,
            String,
            Number,
            Boolean,
            Symbol,
            Bigint,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum RemoteObjectSubtype {
            Array,
            Null,
            Node,
            Regexp,
            Date,
            Map,
            Set,
            Weakmap,
            Weakset,
            Iterator,
            Generator,
            Error,
            Proxy,
            Promise,
            Typedarray,
            Arraybuffer,
            Dataview,
            Webassemblymemory,
            Wasmvalue,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ObjectPreviewType {
            Object,
            Function,
            Undefined,
            String,
            Number,
            Boolean,
            Symbol,
            Bigint,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ObjectPreviewSubtype {
            Array,
            Null,
            Node,
            Regexp,
            Date,
            Map,
            Set,
            Weakmap,
            Weakset,
            Iterator,
            Generator,
            Error,
            Proxy,
            Promise,
            Typedarray,
            Arraybuffer,
            Dataview,
            Webassemblymemory,
            Wasmvalue,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum PropertyPreviewType {
            Object,
            Function,
            Undefined,
            String,
            Number,
            Boolean,
            Symbol,
            Accessor,
            Bigint,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum PropertyPreviewSubtype {
            Array,
            Null,
            Node,
            Regexp,
            Date,
            Map,
            Set,
            Weakmap,
            Weakset,
            Iterator,
            Generator,
            Error,
            Proxy,
            Promise,
            Typedarray,
            Arraybuffer,
            Dataview,
            Webassemblymemory,
            Wasmvalue,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ConsoleAPICalledEventTypeOption {
            Log,
            Debug,
            Info,
            Error,
            Warning,
            Dir,
            Dirxml,
            Table,
            Trace,
            Clear,
            StartGroup,
            StartGroupCollapsed,
            EndGroup,
            Assert,
            Profile,
            ProfileEnd,
            Count,
            TimeEnd,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoteObject {
            pub Type: RemoteObjectType,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub subtype: Option<RemoteObjectSubtype>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub class_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub value: Option<Json>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub unserializable_value: Option<UnserializableValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub object_id: Option<RemoteObjectId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub preview: Option<ObjectPreview>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub custom_preview: Option<CustomPreview>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CustomPreview {
            #[serde(default)]
            pub header: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub body_getter_id: Option<RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ObjectPreview {
            pub Type: ObjectPreviewType,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub subtype: Option<ObjectPreviewSubtype>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub description: Option<String>,
            #[serde(default)]
            pub overflow: bool,
            pub properties: Vec<PropertyPreview>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub entries: Option<Vec<EntryPreview>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PropertyPreview {
            #[serde(default)]
            pub name: String,
            pub Type: PropertyPreviewType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub value: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub value_preview: Option<ObjectPreview>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub subtype: Option<PropertyPreviewSubtype>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EntryPreview {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub key: Option<ObjectPreview>,
            pub value: ObjectPreview,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PropertyDescriptor {
            #[serde(default)]
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub value: Option<RemoteObject>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub writable: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub get: Option<RemoteObject>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub set: Option<RemoteObject>,
            #[serde(default)]
            pub configurable: bool,
            #[serde(default)]
            pub enumerable: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub was_thrown: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub is_own: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub symbol: Option<RemoteObject>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct InternalPropertyDescriptor {
            #[serde(default)]
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub value: Option<RemoteObject>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PrivatePropertyDescriptor {
            #[serde(default)]
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub value: Option<RemoteObject>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub get: Option<RemoteObject>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub set: Option<RemoteObject>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CallArgument {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub value: Option<Json>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub unserializable_value: Option<UnserializableValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub object_id: Option<RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ExecutionContextDescription {
            pub id: ExecutionContextId,
            #[serde(default)]
            pub origin: String,
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub unique_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ExceptionDetails {
            #[serde(default)]
            pub exception_id: JsUInt,
            #[serde(default)]
            pub text: String,
            #[serde(default)]
            pub line_number: JsUInt,
            #[serde(default)]
            pub column_number: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub script_id: Option<ScriptId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stack_trace: Option<StackTrace>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub exception: Option<RemoteObject>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub execution_context_id: Option<ExecutionContextId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CallFrame {
            #[serde(default)]
            pub function_name: String,
            pub script_id: ScriptId,
            #[serde(default)]
            pub url: String,
            #[serde(default)]
            pub line_number: JsUInt,
            #[serde(default)]
            pub column_number: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StackTrace {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub description: Option<String>,
            pub call_frames: Vec<CallFrame>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub parent: Option<Box<StackTrace>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub parent_id: Option<StackTraceId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StackTraceId {
            #[serde(default)]
            pub id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub debugger_id: Option<UniqueDebuggerId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AwaitPromise {
            pub promise_object_id: RemoteObjectId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub return_by_value: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub generate_preview: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CallFunctionOn {
            #[serde(default)]
            pub function_declaration: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub object_id: Option<RemoteObjectId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub arguments: Option<Vec<CallArgument>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub silent: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub return_by_value: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub generate_preview: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub user_gesture: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub await_promise: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub execution_context_id: Option<ExecutionContextId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub object_group: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub throw_on_side_effect: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CompileScript {
            #[serde(default)]
            pub expression: String,
            #[serde(default)]
            pub source_url: String,
            #[serde(default)]
            pub persist_script: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub execution_context_id: Option<ExecutionContextId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DiscardConsoleEntries(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Evaluate {
            #[serde(default)]
            pub expression: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub object_group: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub include_command_line_api: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub silent: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub context_id: Option<ExecutionContextId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub return_by_value: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub generate_preview: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub user_gesture: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub await_promise: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub throw_on_side_effect: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub timeout: Option<TimeDelta>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub disable_breaks: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub repl_mode: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub allow_unsafe_eval_blocked_by_csp: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub unique_context_id: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetIsolateId(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetHeapUsage(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetProperties {
            pub object_id: RemoteObjectId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub own_properties: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub accessor_properties_only: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub generate_preview: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub non_indexed_properties_only: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GlobalLexicalScopeNames {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub execution_context_id: Option<ExecutionContextId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct QueryObjects {
            pub protoType_object_id: RemoteObjectId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub object_group: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReleaseObject {
            pub object_id: RemoteObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReleaseObjectGroup {
            #[serde(default)]
            pub object_group: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RunIfWaitingForDebugger(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RunScript {
            pub script_id: ScriptId,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub execution_context_id: Option<ExecutionContextId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub object_group: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub silent: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub include_command_line_api: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub return_by_value: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub generate_preview: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub await_promise: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAsyncCallStackDepth {
            #[serde(default)]
            pub max_depth: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCustomObjectFormatterEnabled {
            #[serde(default)]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetMaxCallStackSizeToCapture {
            #[serde(default)]
            pub size: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TerminateExecution(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddBinding {
            #[serde(default)]
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub execution_context_id: Option<ExecutionContextId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub execution_context_name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveBinding {
            #[serde(default)]
            pub name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AwaitPromiseReturnObject {
            pub result: RemoteObject,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub exception_details: Option<ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CallFunctionOnReturnObject {
            pub result: RemoteObject,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub exception_details: Option<ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CompileScriptReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub script_id: Option<ScriptId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub exception_details: Option<ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DiscardConsoleEntriesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EvaluateReturnObject {
            pub result: RemoteObject,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub exception_details: Option<ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetIsolateIdReturnObject {
            #[serde(default)]
            pub id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetHeapUsageReturnObject {
            #[serde(default)]
            pub used_size: JsFloat,
            #[serde(default)]
            pub total_size: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetPropertiesReturnObject {
            pub result: Vec<PropertyDescriptor>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub internal_properties: Option<Vec<InternalPropertyDescriptor>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub private_properties: Option<Vec<PrivatePropertyDescriptor>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub exception_details: Option<ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GlobalLexicalScopeNamesReturnObject {
            pub names: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct QueryObjectsReturnObject {
            pub objects: RemoteObject,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReleaseObjectReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReleaseObjectGroupReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RunIfWaitingForDebuggerReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RunScriptReturnObject {
            pub result: RemoteObject,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub exception_details: Option<ExceptionDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAsyncCallStackDepthReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCustomObjectFormatterEnabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetMaxCallStackSizeToCaptureReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TerminateExecutionReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddBindingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveBindingReturnObject {}
        impl Method for AwaitPromise {
            const NAME: &'static str = "Runtime.awaitPromise";
            type ReturnObject = AwaitPromiseReturnObject;
        }
        impl Method for CallFunctionOn {
            const NAME: &'static str = "Runtime.callFunctionOn";
            type ReturnObject = CallFunctionOnReturnObject;
        }
        impl Method for CompileScript {
            const NAME: &'static str = "Runtime.compileScript";
            type ReturnObject = CompileScriptReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Runtime.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for DiscardConsoleEntries {
            const NAME: &'static str = "Runtime.discardConsoleEntries";
            type ReturnObject = DiscardConsoleEntriesReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Runtime.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for Evaluate {
            const NAME: &'static str = "Runtime.evaluate";
            type ReturnObject = EvaluateReturnObject;
        }
        impl Method for GetIsolateId {
            const NAME: &'static str = "Runtime.getIsolateId";
            type ReturnObject = GetIsolateIdReturnObject;
        }
        impl Method for GetHeapUsage {
            const NAME: &'static str = "Runtime.getHeapUsage";
            type ReturnObject = GetHeapUsageReturnObject;
        }
        impl Method for GetProperties {
            const NAME: &'static str = "Runtime.getProperties";
            type ReturnObject = GetPropertiesReturnObject;
        }
        impl Method for GlobalLexicalScopeNames {
            const NAME: &'static str = "Runtime.globalLexicalScopeNames";
            type ReturnObject = GlobalLexicalScopeNamesReturnObject;
        }
        impl Method for QueryObjects {
            const NAME: &'static str = "Runtime.queryObjects";
            type ReturnObject = QueryObjectsReturnObject;
        }
        impl Method for ReleaseObject {
            const NAME: &'static str = "Runtime.releaseObject";
            type ReturnObject = ReleaseObjectReturnObject;
        }
        impl Method for ReleaseObjectGroup {
            const NAME: &'static str = "Runtime.releaseObjectGroup";
            type ReturnObject = ReleaseObjectGroupReturnObject;
        }
        impl Method for RunIfWaitingForDebugger {
            const NAME: &'static str = "Runtime.runIfWaitingForDebugger";
            type ReturnObject = RunIfWaitingForDebuggerReturnObject;
        }
        impl Method for RunScript {
            const NAME: &'static str = "Runtime.runScript";
            type ReturnObject = RunScriptReturnObject;
        }
        impl Method for SetAsyncCallStackDepth {
            const NAME: &'static str = "Runtime.setAsyncCallStackDepth";
            type ReturnObject = SetAsyncCallStackDepthReturnObject;
        }
        impl Method for SetCustomObjectFormatterEnabled {
            const NAME: &'static str = "Runtime.setCustomObjectFormatterEnabled";
            type ReturnObject = SetCustomObjectFormatterEnabledReturnObject;
        }
        impl Method for SetMaxCallStackSizeToCapture {
            const NAME: &'static str = "Runtime.setMaxCallStackSizeToCapture";
            type ReturnObject = SetMaxCallStackSizeToCaptureReturnObject;
        }
        impl Method for TerminateExecution {
            const NAME: &'static str = "Runtime.terminateExecution";
            type ReturnObject = TerminateExecutionReturnObject;
        }
        impl Method for AddBinding {
            const NAME: &'static str = "Runtime.addBinding";
            type ReturnObject = AddBindingReturnObject;
        }
        impl Method for RemoveBinding {
            const NAME: &'static str = "Runtime.removeBinding";
            type ReturnObject = RemoveBindingReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct BindingCalledEvent {
                pub params: BindingCalledEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct BindingCalledEventParams {
                #[serde(default)]
                pub name: String,
                #[serde(default)]
                pub payload: String,
                pub execution_context_id: super::ExecutionContextId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ConsoleAPICalledEvent {
                pub params: ConsoleAPICalledEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ConsoleAPICalledEventParams {
                pub Type: super::ConsoleAPICalledEventTypeOption,
                pub args: Vec<super::RemoteObject>,
                pub execution_context_id: super::ExecutionContextId,
                pub timestamp: super::Timestamp,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub stack_trace: Option<super::StackTrace>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub context: Option<String>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ExceptionRevokedEvent {
                pub params: ExceptionRevokedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ExceptionRevokedEventParams {
                #[serde(default)]
                pub reason: String,
                #[serde(default)]
                pub exception_id: JsUInt,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ExceptionThrownEvent {
                pub params: ExceptionThrownEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ExceptionThrownEventParams {
                pub timestamp: super::Timestamp,
                pub exception_details: super::ExceptionDetails,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ExecutionContextCreatedEvent {
                pub params: ExecutionContextCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ExecutionContextCreatedEventParams {
                pub context: super::ExecutionContextDescription,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ExecutionContextDestroyedEvent {
                pub params: ExecutionContextDestroyedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ExecutionContextDestroyedEventParams {
                pub execution_context_id: super::ExecutionContextId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ExecutionContextsClearedEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct InspectRequestedEvent {
                pub params: InspectRequestedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct InspectRequestedEventParams {
                pub object: super::RemoteObject,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub execution_context_id: Option<super::ExecutionContextId>,
            }
        }
    }
    pub mod Schema {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Domain {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub version: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetDomains(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetDomainsReturnObject {
            pub domains: Vec<Domain>,
        }
        impl Method for GetDomains {
            const NAME: &'static str = "Schema.getDomains";
            type ReturnObject = GetDomainsReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod Accessibility {
        use super::types::*;
        use super::Page;
        use super::Runtime;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type AXNodeId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum AXValueType {
            Boolean,
            Tristate,
            BooleanOrUndefined,
            Idref,
            IdrefList,
            Integer,
            Node,
            NodeList,
            Number,
            String,
            ComputedString,
            Token,
            TokenList,
            DomRelation,
            Role,
            InternalRole,
            ValueUndefined,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum AXValueSourceType {
            Attribute,
            Implicit,
            Style,
            Contents,
            Placeholder,
            RelatedElement,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum AXValueNativeSourceType {
            Description,
            Figcaption,
            Label,
            Labelfor,
            Labelwrapped,
            Legend,
            Rubyannotation,
            Tablecaption,
            Title,
            Other,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum AXPropertyName {
            Busy,
            Disabled,
            Editable,
            Focusable,
            Focused,
            Hidden,
            HiddenRoot,
            Invalid,
            Keyshortcuts,
            Settable,
            Roledescription,
            Live,
            Atomic,
            Relevant,
            Root,
            Autocomplete,
            HasPopup,
            Level,
            Multiselectable,
            Orientation,
            Multiline,
            Readonly,
            Required,
            Valuemin,
            Valuemax,
            Valuetext,
            Checked,
            Expanded,
            Modal,
            Pressed,
            Selected,
            Activedescendant,
            Controls,
            Describedby,
            Details,
            Errormessage,
            Flowto,
            Labelledby,
            Owns,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AXValueSource {
            pub Type: AXValueSourceType,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub value: Option<AXValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub attribute: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub attribute_value: Option<AXValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub superseded: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub native_source: Option<AXValueNativeSourceType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub native_source_value: Option<AXValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub invalid: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub invalid_reason: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AXRelatedNode {
            pub backend_dom_node_id: DOM::BackendNodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub idref: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub text: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AXProperty {
            pub name: AXPropertyName,
            pub value: AXValue,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AXValue {
            pub Type: AXValueType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub value: Option<Json>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub related_nodes: Option<Vec<AXRelatedNode>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub sources: Option<Vec<AXValueSource>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AXNode {
            pub node_id: AXNodeId,
            #[serde(default)]
            pub ignored: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ignored_reasons: Option<Vec<AXProperty>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub role: Option<AXValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: Option<AXValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub description: Option<AXValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub value: Option<AXValue>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub properties: Option<Vec<AXProperty>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub parent_id: Option<AXNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub child_ids: Option<Vec<AXNodeId>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_dom_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub frame_id: Option<Page::FrameId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetPartialAXTree {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<DOM::NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub object_id: Option<Runtime::RemoteObjectId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub fetch_relatives: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetFullAXTree {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub depth: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub max_depth: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub frame_id: Option<Page::FrameId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetRootAXNode {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub frame_id: Option<Page::FrameId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAXNodeAndAncestors {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<DOM::NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub object_id: Option<Runtime::RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetChildAXNodes {
            pub id: AXNodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub frame_id: Option<Page::FrameId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct QueryAXTree {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<DOM::NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub object_id: Option<Runtime::RemoteObjectId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub accessible_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub role: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetPartialAXTreeReturnObject {
            pub nodes: Vec<AXNode>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetFullAXTreeReturnObject {
            pub nodes: Vec<AXNode>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetRootAXNodeReturnObject {
            pub node: AXNode,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAXNodeAndAncestorsReturnObject {
            pub nodes: Vec<AXNode>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetChildAXNodesReturnObject {
            pub nodes: Vec<AXNode>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct QueryAXTreeReturnObject {
            pub nodes: Vec<AXNode>,
        }
        impl Method for Disable {
            const NAME: &'static str = "Accessibility.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Accessibility.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetPartialAXTree {
            const NAME: &'static str = "Accessibility.getPartialAXTree";
            type ReturnObject = GetPartialAXTreeReturnObject;
        }
        impl Method for GetFullAXTree {
            const NAME: &'static str = "Accessibility.getFullAXTree";
            type ReturnObject = GetFullAXTreeReturnObject;
        }
        impl Method for GetRootAXNode {
            const NAME: &'static str = "Accessibility.getRootAXNode";
            type ReturnObject = GetRootAXNodeReturnObject;
        }
        impl Method for GetAXNodeAndAncestors {
            const NAME: &'static str = "Accessibility.getAXNodeAndAncestors";
            type ReturnObject = GetAXNodeAndAncestorsReturnObject;
        }
        impl Method for GetChildAXNodes {
            const NAME: &'static str = "Accessibility.getChildAXNodes";
            type ReturnObject = GetChildAXNodesReturnObject;
        }
        impl Method for QueryAXTree {
            const NAME: &'static str = "Accessibility.queryAXTree";
            type ReturnObject = QueryAXTreeReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LoadCompleteEvent {
                pub params: LoadCompleteEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct LoadCompleteEventParams {
                pub root: super::AXNode,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodesUpdatedEvent {
                pub params: NodesUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct NodesUpdatedEventParams {
                pub nodes: Vec<super::AXNode>,
            }
        }
    }
    pub mod Animation {
        use super::types::*;
        use super::Runtime;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum AnimationType {
            CssTransition,
            CssAnimation,
            WebAnimation,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Animation {
            #[serde(default)]
            pub id: String,
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub paused_state: bool,
            #[serde(default)]
            pub play_state: String,
            #[serde(default)]
            pub playback_rate: JsFloat,
            #[serde(default)]
            pub start_time: JsFloat,
            #[serde(default)]
            pub current_time: JsFloat,
            pub Type: AnimationType,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub source: Option<AnimationEffect>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub css_id: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AnimationEffect {
            #[serde(default)]
            pub delay: JsFloat,
            #[serde(default)]
            pub end_delay: JsFloat,
            #[serde(default)]
            pub iteration_start: JsFloat,
            #[serde(default)]
            pub iterations: JsFloat,
            #[serde(default)]
            pub duration: JsFloat,
            #[serde(default)]
            pub direction: String,
            #[serde(default)]
            pub fill: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub keyframes_rule: Option<KeyframesRule>,
            #[serde(default)]
            pub easing: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct KeyframesRule {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub name: Option<String>,
            pub keyframes: Vec<KeyframeStyle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct KeyframeStyle {
            #[serde(default)]
            pub offset: String,
            #[serde(default)]
            pub easing: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCurrentTime {
            #[serde(default)]
            pub id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetPlaybackRate(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReleaseAnimations {
            #[serde(default)]
            pub animations: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResolveAnimation {
            #[serde(default)]
            pub animation_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SeekAnimations {
            #[serde(default)]
            pub animations: Vec<String>,
            #[serde(default)]
            pub current_time: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPaused {
            #[serde(default)]
            pub animations: Vec<String>,
            #[serde(default)]
            pub paused: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPlaybackRate {
            #[serde(default)]
            pub playback_rate: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetTiming {
            #[serde(default)]
            pub animation_id: String,
            #[serde(default)]
            pub duration: JsFloat,
            #[serde(default)]
            pub delay: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCurrentTimeReturnObject {
            #[serde(default)]
            pub current_time: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetPlaybackRateReturnObject {
            #[serde(default)]
            pub playback_rate: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReleaseAnimationsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResolveAnimationReturnObject {
            pub remote_object: Runtime::RemoteObject,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SeekAnimationsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPausedReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPlaybackRateReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetTimingReturnObject {}
        impl Method for Disable {
            const NAME: &'static str = "Animation.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Animation.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetCurrentTime {
            const NAME: &'static str = "Animation.getCurrentTime";
            type ReturnObject = GetCurrentTimeReturnObject;
        }
        impl Method for GetPlaybackRate {
            const NAME: &'static str = "Animation.getPlaybackRate";
            type ReturnObject = GetPlaybackRateReturnObject;
        }
        impl Method for ReleaseAnimations {
            const NAME: &'static str = "Animation.releaseAnimations";
            type ReturnObject = ReleaseAnimationsReturnObject;
        }
        impl Method for ResolveAnimation {
            const NAME: &'static str = "Animation.resolveAnimation";
            type ReturnObject = ResolveAnimationReturnObject;
        }
        impl Method for SeekAnimations {
            const NAME: &'static str = "Animation.seekAnimations";
            type ReturnObject = SeekAnimationsReturnObject;
        }
        impl Method for SetPaused {
            const NAME: &'static str = "Animation.setPaused";
            type ReturnObject = SetPausedReturnObject;
        }
        impl Method for SetPlaybackRate {
            const NAME: &'static str = "Animation.setPlaybackRate";
            type ReturnObject = SetPlaybackRateReturnObject;
        }
        impl Method for SetTiming {
            const NAME: &'static str = "Animation.setTiming";
            type ReturnObject = SetTimingReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AnimationCanceledEvent {
                pub params: AnimationCanceledEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AnimationCanceledEventParams {
                #[serde(default)]
                pub id: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AnimationCreatedEvent {
                pub params: AnimationCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AnimationCreatedEventParams {
                #[serde(default)]
                pub id: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AnimationStartedEvent {
                pub params: AnimationStartedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AnimationStartedEventParams {
                pub animation: super::Animation,
            }
        }
    }
    pub mod Audits {
        use super::types::*;
        use super::Network;
        use super::Page;
        use super::Runtime;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type IssueId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum SameSiteCookieExclusionReason {
            ExcludeSameSiteUnspecifiedTreatedAsLax,
            ExcludeSameSiteNoneInsecure,
            ExcludeSameSiteLax,
            ExcludeSameSiteStrict,
            ExcludeInvalidSameParty,
            ExcludeSamePartyCrossPartyContext,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum SameSiteCookieWarningReason {
            WarnSameSiteUnspecifiedCrossSiteContext,
            WarnSameSiteNoneInsecure,
            WarnSameSiteUnspecifiedLaxAllowUnsafe,
            WarnSameSiteStrictLaxDowngradeStrict,
            WarnSameSiteStrictCrossDowngradeStrict,
            WarnSameSiteStrictCrossDowngradeLax,
            WarnSameSiteLaxCrossDowngradeStrict,
            WarnSameSiteLaxCrossDowngradeLax,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum SameSiteCookieOperation {
            SetCookie,
            ReadCookie,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum MixedContentResolutionStatus {
            MixedContentBlocked,
            MixedContentAutomaticallyUpgraded,
            MixedContentWarning,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum MixedContentResourceType {
            Audio,
            Beacon,
            CspReport,
            Download,
            EventSource,
            Favicon,
            Font,
            Form,
            Frame,
            Image,
            Import,
            Manifest,
            Ping,
            PluginData,
            PluginResource,
            Prefetch,
            Resource,
            Script,
            ServiceWorker,
            SharedWorker,
            Stylesheet,
            Track,
            Video,
            Worker,
            XmlHttpRequest,
            Xslt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum BlockedByResponseReason {
            CoepFrameResourceNeedsCoepHeader,
            CoopSandboxedIFrameCannotNavigateToCoopPage,
            CorpNotSameOrigin,
            CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
            CorpNotSameSite,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum HeavyAdResolutionStatus {
            HeavyAdBlocked,
            HeavyAdWarning,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum HeavyAdReason {
            NetworkTotalLimit,
            CpuTotalLimit,
            CpuPeakLimit,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ContentSecurityPolicyViolationType {
            KInlineViolation,
            KEvalViolation,
            KUrlViolation,
            KTrustedTypesSinkViolation,
            KTrustedTypesPolicyViolation,
            KWasmEvalViolation,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum SharedArrayBufferIssueType {
            TransferIssue,
            CreationIssue,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum TwaQualityEnforcementViolationType {
            KHttpError,
            KUnavailableOffline,
            KDigitalAssetLinks,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum AttributionReportingIssueType {
            PermissionPolicyDisabled,
            InvalidAttributionSourceEventId,
            InvalidAttributionData,
            AttributionSourceUntrustworthyOrigin,
            AttributionUntrustworthyOrigin,
            AttributionTriggerDataTooLarge,
            AttributionEventSourceTriggerDataTooLarge,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum GenericIssueErrorType {
            CrossOriginPortalPostMessageError,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum InspectorIssueCode {
            SameSiteCookieIssue,
            MixedContentIssue,
            BlockedByResponseIssue,
            HeavyAdIssue,
            ContentSecurityPolicyIssue,
            SharedArrayBufferIssue,
            TrustedWebActivityIssue,
            LowTextContrastIssue,
            CorsIssue,
            AttributionReportingIssue,
            QuirksModeIssue,
            NavigatorUserAgentIssue,
            WasmCrossOriginModuleSharingIssue,
            GenericIssue,
            DeprecationIssue,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum GetEncodedResponseEncodingOption {
            Webp,
            Jpeg,
            Png,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AffectedCookie {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub path: String,
            #[serde(default)]
            pub domain: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AffectedRequest {
            pub request_id: Network::RequestId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AffectedFrame {
            pub frame_id: Page::FrameId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SameSiteCookieIssueDetails {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cookie: Option<AffectedCookie>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub raw_cookie_line: Option<String>,
            pub cookie_warning_reasons: Vec<SameSiteCookieWarningReason>,
            pub cookie_exclusion_reasons: Vec<SameSiteCookieExclusionReason>,
            pub operation: SameSiteCookieOperation,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub site_for_cookies: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub cookie_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub request: Option<AffectedRequest>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct MixedContentIssueDetails {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub resource_Type: Option<MixedContentResourceType>,
            pub resolution_status: MixedContentResolutionStatus,
            #[serde(default)]
            pub insecure_url: String,
            #[serde(default)]
            pub main_resource_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub request: Option<AffectedRequest>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub frame: Option<AffectedFrame>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BlockedByResponseIssueDetails {
            pub request: AffectedRequest,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub parent_frame: Option<AffectedFrame>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub blocked_frame: Option<AffectedFrame>,
            pub reason: BlockedByResponseReason,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HeavyAdIssueDetails {
            pub resolution: HeavyAdResolutionStatus,
            pub reason: HeavyAdReason,
            pub frame: AffectedFrame,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SourceCodeLocation {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub script_id: Option<Runtime::ScriptId>,
            #[serde(default)]
            pub url: String,
            #[serde(default)]
            pub line_number: JsUInt,
            #[serde(default)]
            pub column_number: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContentSecurityPolicyIssueDetails {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub blocked_url: Option<String>,
            #[serde(default)]
            pub violated_directive: String,
            #[serde(default)]
            pub is_report_only: bool,
            pub content_security_policy_violation_Type: ContentSecurityPolicyViolationType,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub frame_ancestor: Option<AffectedFrame>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub source_code_location: Option<SourceCodeLocation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub violating_node_id: Option<DOM::BackendNodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SharedArrayBufferIssueDetails {
            pub source_code_location: SourceCodeLocation,
            #[serde(default)]
            pub is_warning: bool,
            pub Type: SharedArrayBufferIssueType,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TrustedWebActivityIssueDetails {
            #[serde(default)]
            pub url: String,
            pub violation_Type: TwaQualityEnforcementViolationType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub http_status_code: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub package_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub signature: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LowTextContrastIssueDetails {
            pub violating_node_id: DOM::BackendNodeId,
            #[serde(default)]
            pub violating_node_selector: String,
            #[serde(default)]
            pub contrast_ratio: JsFloat,
            #[serde(default)]
            pub threshold_aa: JsFloat,
            #[serde(default)]
            pub threshold_aaa: JsFloat,
            #[serde(default)]
            pub font_size: String,
            #[serde(default)]
            pub font_weight: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CorsIssueDetails {
            pub cors_error_status: Network::CorsErrorStatus,
            #[serde(default)]
            pub is_warning: bool,
            pub request: AffectedRequest,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub location: Option<SourceCodeLocation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub initiator_origin: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub resource_ip_address_space: Option<Network::IPAddressSpace>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub client_security_state: Option<Network::ClientSecurityState>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AttributionReportingIssueDetails {
            pub violation_Type: AttributionReportingIssueType,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub frame: Option<AffectedFrame>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub request: Option<AffectedRequest>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub violating_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub invalid_parameter: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct QuirksModeIssueDetails {
            #[serde(default)]
            pub is_limited_quirks_mode: bool,
            pub document_node_id: DOM::BackendNodeId,
            #[serde(default)]
            pub url: String,
            pub frame_id: Page::FrameId,
            pub loader_id: Network::LoaderId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct NavigatorUserAgentIssueDetails {
            #[serde(default)]
            pub url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub location: Option<SourceCodeLocation>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct WasmCrossOriginModuleSharingIssueDetails {
            #[serde(default)]
            pub wasm_module_url: String,
            #[serde(default)]
            pub source_origin: String,
            #[serde(default)]
            pub target_origin: String,
            #[serde(default)]
            pub is_warning: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GenericIssueDetails {
            pub error_Type: GenericIssueErrorType,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub frame_id: Option<Page::FrameId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeprecationIssueDetails {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub affected_frame: Option<AffectedFrame>,
            pub source_code_location: SourceCodeLocation,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub message: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct InspectorIssueDetails {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub same_site_cookie_issue_details: Option<SameSiteCookieIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mixed_content_issue_details: Option<MixedContentIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub blocked_by_response_issue_details: Option<BlockedByResponseIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub heavy_ad_issue_details: Option<HeavyAdIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub content_security_policy_issue_details: Option<ContentSecurityPolicyIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub shared_array_buffer_issue_details: Option<SharedArrayBufferIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub twa_quality_enforcement_details: Option<TrustedWebActivityIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub low_text_contrast_issue_details: Option<LowTextContrastIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cors_issue_details: Option<CorsIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub attribution_reporting_issue_details: Option<AttributionReportingIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub quirks_mode_issue_details: Option<QuirksModeIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub navigator_user_agent_issue_details: Option<NavigatorUserAgentIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub wasm_cross_origin_module_sharing_issue:
                Option<WasmCrossOriginModuleSharingIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub generic_issue_details: Option<GenericIssueDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub deprecation_issue_details: Option<DeprecationIssueDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct InspectorIssue {
            pub code: InspectorIssueCode,
            pub details: InspectorIssueDetails,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub issue_id: Option<IssueId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetEncodedResponse {
            pub request_id: Network::RequestId,
            pub encoding: GetEncodedResponseEncodingOption,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub quality: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub size_only: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CheckContrast {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub report_aaa: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetEncodedResponseReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub body: Option<String>,
            #[serde(default)]
            pub original_size: JsUInt,
            #[serde(default)]
            pub encoded_size: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CheckContrastReturnObject {}
        impl Method for GetEncodedResponse {
            const NAME: &'static str = "Audits.getEncodedResponse";
            type ReturnObject = GetEncodedResponseReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Audits.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Audits.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for CheckContrast {
            const NAME: &'static str = "Audits.checkContrast";
            type ReturnObject = CheckContrastReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct IssueAddedEvent {
                pub params: IssueAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct IssueAddedEventParams {
                pub issue: super::InspectorIssue,
            }
        }
    }
    pub mod BackgroundService {
        use super::types::*;
        use super::Network;
        use super::ServiceWorker;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ServiceName {
            BackgroundFetch,
            BackgroundSync,
            PushMessaging,
            Notifications,
            PaymentHandler,
            PeriodicBackgroundSync,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EventMetadata {
            #[serde(default)]
            pub key: String,
            #[serde(default)]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BackgroundServiceEvent {
            pub timestamp: Network::TimeSinceEpoch,
            #[serde(default)]
            pub origin: String,
            pub service_worker_registration_id: ServiceWorker::RegistrationID,
            pub service: ServiceName,
            #[serde(default)]
            pub event_name: String,
            #[serde(default)]
            pub instance_id: String,
            pub event_metadata: Vec<EventMetadata>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartObserving {
            pub service: ServiceName,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopObserving {
            pub service: ServiceName,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetRecording {
            #[serde(default)]
            pub should_record: bool,
            pub service: ServiceName,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearEvents {
            pub service: ServiceName,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartObservingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopObservingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetRecordingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearEventsReturnObject {}
        impl Method for StartObserving {
            const NAME: &'static str = "BackgroundService.startObserving";
            type ReturnObject = StartObservingReturnObject;
        }
        impl Method for StopObserving {
            const NAME: &'static str = "BackgroundService.stopObserving";
            type ReturnObject = StopObservingReturnObject;
        }
        impl Method for SetRecording {
            const NAME: &'static str = "BackgroundService.setRecording";
            type ReturnObject = SetRecordingReturnObject;
        }
        impl Method for ClearEvents {
            const NAME: &'static str = "BackgroundService.clearEvents";
            type ReturnObject = ClearEventsReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RecordingStateChangedEvent {
                pub params: RecordingStateChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct RecordingStateChangedEventParams {
                #[serde(default)]
                pub is_recording: bool,
                pub service: super::ServiceName,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct BackgroundServiceEventReceivedEvent {
                pub params: BackgroundServiceEventReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct BackgroundServiceEventReceivedEventParams {
                pub background_service_event: super::BackgroundServiceEvent,
            }
        }
    }
    pub mod Browser {
        use super::types::*;
        use super::Target;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type BrowserContextID = String;
        pub type WindowID = JsUInt;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum WindowState {
            Normal,
            Minimized,
            Maximized,
            Fullscreen,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum PermissionType {
            AccessibilityEvents,
            AudioCapture,
            BackgroundSync,
            BackgroundFetch,
            ClipboardReadWrite,
            ClipboardSanitizedWrite,
            DisplayCapture,
            DurableStorage,
            Flash,
            Geolocation,
            Midi,
            MidiSysex,
            Nfc,
            Notifications,
            PaymentHandler,
            PeriodicBackgroundSync,
            ProtectedMediaIdentifier,
            Sensors,
            VideoCapture,
            VideoCapturePanTiltZoom,
            IdleDetection,
            WakeLockScreen,
            WakeLockSystem,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum PermissionSetting {
            Granted,
            Denied,
            Prompt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum BrowserCommandId {
            OpenTabSearch,
            CloseTabSearch,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum SetDownloadBehaviorBehaviorOption {
            Deny,
            Allow,
            AllowAndName,
            Default,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum DownloadProgressEventStateOption {
            InProgress,
            Completed,
            Canceled,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Bounds {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub left: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub top: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub width: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub height: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub window_state: Option<WindowState>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PermissionDescriptor {
            #[serde(default)]
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub sysex: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub user_visible_only: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub allow_without_sanitization: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub pan_tilt_zoom: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Bucket {
            #[serde(default)]
            pub low: JsUInt,
            #[serde(default)]
            pub high: JsUInt,
            #[serde(default)]
            pub count: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Histogram {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub sum: JsUInt,
            #[serde(default)]
            pub count: JsUInt,
            pub buckets: Vec<Bucket>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPermission {
            pub permission: PermissionDescriptor,
            pub setting: PermissionSetting,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub origin: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub browser_context_id: Option<BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GrantPermissions {
            pub permissions: Vec<PermissionType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub origin: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub browser_context_id: Option<BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResetPermissions {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub browser_context_id: Option<BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDownloadBehavior {
            pub behavior: SetDownloadBehaviorBehaviorOption,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub browser_context_id: Option<BrowserContextID>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub download_path: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub events_enabled: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CancelDownload {
            #[serde(default)]
            pub guid: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub browser_context_id: Option<BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Close(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Crash(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CrashGpuProcess(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetVersion(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBrowserCommandLine(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetHistograms {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub query: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub delta: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetHistogram {
            #[serde(default)]
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub delta: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetWindowBounds {
            pub window_id: WindowID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetWindowForTarget {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub target_id: Option<Target::TargetID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetWindowBounds {
            pub window_id: WindowID,
            pub bounds: Bounds,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDockTile {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub badge_label: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub image: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ExecuteBrowserCommand {
            pub command_id: BrowserCommandId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPermissionReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GrantPermissionsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResetPermissionsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDownloadBehaviorReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CancelDownloadReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CloseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CrashReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CrashGpuProcessReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetVersionReturnObject {
            #[serde(default)]
            pub protocol_version: String,
            #[serde(default)]
            pub product: String,
            #[serde(default)]
            pub revision: String,
            #[serde(default)]
            pub user_agent: String,
            #[serde(default)]
            pub js_version: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBrowserCommandLineReturnObject {
            pub arguments: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetHistogramsReturnObject {
            pub histograms: Vec<Histogram>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetHistogramReturnObject {
            pub histogram: Histogram,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetWindowBoundsReturnObject {
            pub bounds: Bounds,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetWindowForTargetReturnObject {
            pub window_id: WindowID,
            pub bounds: Bounds,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetWindowBoundsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDockTileReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ExecuteBrowserCommandReturnObject {}
        impl Method for SetPermission {
            const NAME: &'static str = "Browser.setPermission";
            type ReturnObject = SetPermissionReturnObject;
        }
        impl Method for GrantPermissions {
            const NAME: &'static str = "Browser.grantPermissions";
            type ReturnObject = GrantPermissionsReturnObject;
        }
        impl Method for ResetPermissions {
            const NAME: &'static str = "Browser.resetPermissions";
            type ReturnObject = ResetPermissionsReturnObject;
        }
        impl Method for SetDownloadBehavior {
            const NAME: &'static str = "Browser.setDownloadBehavior";
            type ReturnObject = SetDownloadBehaviorReturnObject;
        }
        impl Method for CancelDownload {
            const NAME: &'static str = "Browser.cancelDownload";
            type ReturnObject = CancelDownloadReturnObject;
        }
        impl Method for Close {
            const NAME: &'static str = "Browser.close";
            type ReturnObject = CloseReturnObject;
        }
        impl Method for Crash {
            const NAME: &'static str = "Browser.crash";
            type ReturnObject = CrashReturnObject;
        }
        impl Method for CrashGpuProcess {
            const NAME: &'static str = "Browser.crashGpuProcess";
            type ReturnObject = CrashGpuProcessReturnObject;
        }
        impl Method for GetVersion {
            const NAME: &'static str = "Browser.getVersion";
            type ReturnObject = GetVersionReturnObject;
        }
        impl Method for GetBrowserCommandLine {
            const NAME: &'static str = "Browser.getBrowserCommandLine";
            type ReturnObject = GetBrowserCommandLineReturnObject;
        }
        impl Method for GetHistograms {
            const NAME: &'static str = "Browser.getHistograms";
            type ReturnObject = GetHistogramsReturnObject;
        }
        impl Method for GetHistogram {
            const NAME: &'static str = "Browser.getHistogram";
            type ReturnObject = GetHistogramReturnObject;
        }
        impl Method for GetWindowBounds {
            const NAME: &'static str = "Browser.getWindowBounds";
            type ReturnObject = GetWindowBoundsReturnObject;
        }
        impl Method for GetWindowForTarget {
            const NAME: &'static str = "Browser.getWindowForTarget";
            type ReturnObject = GetWindowForTargetReturnObject;
        }
        impl Method for SetWindowBounds {
            const NAME: &'static str = "Browser.setWindowBounds";
            type ReturnObject = SetWindowBoundsReturnObject;
        }
        impl Method for SetDockTile {
            const NAME: &'static str = "Browser.setDockTile";
            type ReturnObject = SetDockTileReturnObject;
        }
        impl Method for ExecuteBrowserCommand {
            const NAME: &'static str = "Browser.executeBrowserCommand";
            type ReturnObject = ExecuteBrowserCommandReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DownloadWillBeginEvent {
                pub params: DownloadWillBeginEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DownloadWillBeginEventParams {
                pub frame_id: super::super::Page::FrameId,
                #[serde(default)]
                pub guid: String,
                #[serde(default)]
                pub url: String,
                #[serde(default)]
                pub suggested_filename: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DownloadProgressEvent {
                pub params: DownloadProgressEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DownloadProgressEventParams {
                #[serde(default)]
                pub guid: String,
                #[serde(default)]
                pub total_bytes: JsFloat,
                #[serde(default)]
                pub received_bytes: JsFloat,
                pub state: super::DownloadProgressEventStateOption,
            }
        }
    }
    pub mod CSS {
        use super::types::*;
        use super::Page;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type StyleSheetId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum StyleSheetOrigin {
            Injected,
            UserAgent,
            Inspector,
            Regular,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum CssMediaSource {
            MediaRule,
            ImportRule,
            LinkedSheet,
            InlineSheet,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PseudoElementMatches {
            pub pseudo_Type: DOM::PseudoType,
            pub matches: Vec<RuleMatch>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct InheritedStyleEntry {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub inline_style: Option<CSSStyle>,
            pub matched_css_rules: Vec<RuleMatch>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RuleMatch {
            pub rule: CSSRule,
            #[serde(default)]
            pub matching_selectors: Vec<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Value {
            #[serde(default)]
            pub text: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub range: Option<SourceRange>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SelectorList {
            pub selectors: Vec<Value>,
            #[serde(default)]
            pub text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CSSStyleSheetHeader {
            pub style_sheet_id: StyleSheetId,
            pub frame_id: Page::FrameId,
            #[serde(default)]
            pub source_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub source_map_url: Option<String>,
            pub origin: StyleSheetOrigin,
            #[serde(default)]
            pub title: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub owner_node: Option<DOM::BackendNodeId>,
            #[serde(default)]
            pub disabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub has_source_url: Option<bool>,
            #[serde(default)]
            pub is_inline: bool,
            #[serde(default)]
            pub is_mutable: bool,
            #[serde(default)]
            pub is_constructed: bool,
            #[serde(default)]
            pub start_line: JsFloat,
            #[serde(default)]
            pub start_column: JsFloat,
            #[serde(default)]
            pub length: JsFloat,
            #[serde(default)]
            pub end_line: JsFloat,
            #[serde(default)]
            pub end_column: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CSSRule {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub style_sheet_id: Option<StyleSheetId>,
            pub selector_list: SelectorList,
            pub origin: StyleSheetOrigin,
            pub style: CSSStyle,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub media: Option<Vec<CSSMedia>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub container_queries: Option<Vec<CSSContainerQuery>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RuleUsage {
            pub style_sheet_id: StyleSheetId,
            #[serde(default)]
            pub start_offset: JsFloat,
            #[serde(default)]
            pub end_offset: JsFloat,
            #[serde(default)]
            pub used: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SourceRange {
            #[serde(default)]
            pub start_line: JsUInt,
            #[serde(default)]
            pub start_column: JsUInt,
            #[serde(default)]
            pub end_line: JsUInt,
            #[serde(default)]
            pub end_column: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ShorthandEntry {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub value: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub important: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CSSComputedStyleProperty {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CSSStyle {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub style_sheet_id: Option<StyleSheetId>,
            pub css_properties: Vec<CSSProperty>,
            pub shorthand_entries: Vec<ShorthandEntry>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub css_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub range: Option<SourceRange>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CSSProperty {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub value: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub important: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub implicit: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub parsed_ok: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub disabled: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub range: Option<SourceRange>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CSSMedia {
            #[serde(default)]
            pub text: String,
            pub source: CssMediaSource,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub source_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub range: Option<SourceRange>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub style_sheet_id: Option<StyleSheetId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub media_list: Option<Vec<MediaQuery>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct MediaQuery {
            pub expressions: Vec<MediaQueryExpression>,
            #[serde(default)]
            pub active: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct MediaQueryExpression {
            #[serde(default)]
            pub value: JsFloat,
            #[serde(default)]
            pub unit: String,
            #[serde(default)]
            pub feature: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub value_range: Option<SourceRange>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub computed_length: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CSSContainerQuery {
            #[serde(default)]
            pub text: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub range: Option<SourceRange>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub style_sheet_id: Option<StyleSheetId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PlatformFontUsage {
            #[serde(default)]
            pub family_name: String,
            #[serde(default)]
            pub is_custom_font: bool,
            #[serde(default)]
            pub glyph_count: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FontVariationAxis {
            #[serde(default)]
            pub tag: String,
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub min_value: JsFloat,
            #[serde(default)]
            pub max_value: JsFloat,
            #[serde(default)]
            pub default_value: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FontFace {
            #[serde(default)]
            pub font_family: String,
            #[serde(default)]
            pub font_style: String,
            #[serde(default)]
            pub font_variant: String,
            #[serde(default)]
            pub font_weight: String,
            #[serde(default)]
            pub font_stretch: String,
            #[serde(default)]
            pub unicode_range: String,
            #[serde(default)]
            pub src: String,
            #[serde(default)]
            pub platform_font_family: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub font_variation_axes: Option<Vec<FontVariationAxis>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CSSKeyframesRule {
            pub animation_name: Value,
            pub keyframes: Vec<CSSKeyframeRule>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CSSKeyframeRule {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub style_sheet_id: Option<StyleSheetId>,
            pub origin: StyleSheetOrigin,
            pub key_text: Value,
            pub style: CSSStyle,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StyleDeclarationEdit {
            pub style_sheet_id: StyleSheetId,
            pub range: SourceRange,
            #[serde(default)]
            pub text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddRule {
            pub style_sheet_id: StyleSheetId,
            #[serde(default)]
            pub rule_text: String,
            pub location: SourceRange,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CollectClassNames {
            pub style_sheet_id: StyleSheetId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CreateStyleSheet {
            pub frame_id: Page::FrameId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ForcePseudoState {
            pub node_id: DOM::NodeId,
            #[serde(default)]
            pub forced_pseudo_classes: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBackgroundColors {
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetComputedStyleForNode {
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetInlineStylesForNode {
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetMatchedStylesForNode {
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetMediaQueries(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetPlatformFontsForNode {
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetStyleSheetText {
            pub style_sheet_id: StyleSheetId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TrackComputedStyleUpdates {
            pub properties_to_track: Vec<CSSComputedStyleProperty>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeComputedStyleUpdates(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetEffectivePropertyValueForNode {
            pub node_id: DOM::NodeId,
            #[serde(default)]
            pub property_name: String,
            #[serde(default)]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetKeyframeKey {
            pub style_sheet_id: StyleSheetId,
            pub range: SourceRange,
            #[serde(default)]
            pub key_text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetMediaText {
            pub style_sheet_id: StyleSheetId,
            pub range: SourceRange,
            #[serde(default)]
            pub text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetContainerQueryText {
            pub style_sheet_id: StyleSheetId,
            pub range: SourceRange,
            #[serde(default)]
            pub text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetRuleSelector {
            pub style_sheet_id: StyleSheetId,
            pub range: SourceRange,
            #[serde(default)]
            pub selector: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetStyleSheetText {
            pub style_sheet_id: StyleSheetId,
            #[serde(default)]
            pub text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetStyleTexts {
            pub edits: Vec<StyleDeclarationEdit>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartRuleUsageTracking(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopRuleUsageTracking(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeCoverageDelta(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetLocalFontsEnabled {
            #[serde(default)]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddRuleReturnObject {
            pub rule: CSSRule,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CollectClassNamesReturnObject {
            pub class_names: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CreateStyleSheetReturnObject {
            pub style_sheet_id: StyleSheetId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ForcePseudoStateReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBackgroundColorsReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub background_colors: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub computed_font_size: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub computed_font_weight: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetComputedStyleForNodeReturnObject {
            pub computed_style: Vec<CSSComputedStyleProperty>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetInlineStylesForNodeReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub inline_style: Option<CSSStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub attributes_style: Option<CSSStyle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetMatchedStylesForNodeReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub inline_style: Option<CSSStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub attributes_style: Option<CSSStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub matched_css_rules: Option<Vec<RuleMatch>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub pseudo_elements: Option<Vec<PseudoElementMatches>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub inherited: Option<Vec<InheritedStyleEntry>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub css_keyframes_rules: Option<Vec<CSSKeyframesRule>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetMediaQueriesReturnObject {
            pub medias: Vec<CSSMedia>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetPlatformFontsForNodeReturnObject {
            pub fonts: Vec<PlatformFontUsage>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetStyleSheetTextReturnObject {
            #[serde(default)]
            pub text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TrackComputedStyleUpdatesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeComputedStyleUpdatesReturnObject {
            pub node_ids: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetEffectivePropertyValueForNodeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetKeyframeKeyReturnObject {
            pub key_text: Value,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetMediaTextReturnObject {
            pub media: CSSMedia,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetContainerQueryTextReturnObject {
            pub container_query: CSSContainerQuery,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetRuleSelectorReturnObject {
            pub selector_list: SelectorList,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetStyleSheetTextReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub source_map_url: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetStyleTextsReturnObject {
            pub styles: Vec<CSSStyle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartRuleUsageTrackingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopRuleUsageTrackingReturnObject {
            pub rule_usage: Vec<RuleUsage>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeCoverageDeltaReturnObject {
            pub coverage: Vec<RuleUsage>,
            #[serde(default)]
            pub timestamp: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetLocalFontsEnabledReturnObject {}
        impl Method for AddRule {
            const NAME: &'static str = "CSS.addRule";
            type ReturnObject = AddRuleReturnObject;
        }
        impl Method for CollectClassNames {
            const NAME: &'static str = "CSS.collectClassNames";
            type ReturnObject = CollectClassNamesReturnObject;
        }
        impl Method for CreateStyleSheet {
            const NAME: &'static str = "CSS.createStyleSheet";
            type ReturnObject = CreateStyleSheetReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "CSS.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "CSS.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for ForcePseudoState {
            const NAME: &'static str = "CSS.forcePseudoState";
            type ReturnObject = ForcePseudoStateReturnObject;
        }
        impl Method for GetBackgroundColors {
            const NAME: &'static str = "CSS.getBackgroundColors";
            type ReturnObject = GetBackgroundColorsReturnObject;
        }
        impl Method for GetComputedStyleForNode {
            const NAME: &'static str = "CSS.getComputedStyleForNode";
            type ReturnObject = GetComputedStyleForNodeReturnObject;
        }
        impl Method for GetInlineStylesForNode {
            const NAME: &'static str = "CSS.getInlineStylesForNode";
            type ReturnObject = GetInlineStylesForNodeReturnObject;
        }
        impl Method for GetMatchedStylesForNode {
            const NAME: &'static str = "CSS.getMatchedStylesForNode";
            type ReturnObject = GetMatchedStylesForNodeReturnObject;
        }
        impl Method for GetMediaQueries {
            const NAME: &'static str = "CSS.getMediaQueries";
            type ReturnObject = GetMediaQueriesReturnObject;
        }
        impl Method for GetPlatformFontsForNode {
            const NAME: &'static str = "CSS.getPlatformFontsForNode";
            type ReturnObject = GetPlatformFontsForNodeReturnObject;
        }
        impl Method for GetStyleSheetText {
            const NAME: &'static str = "CSS.getStyleSheetText";
            type ReturnObject = GetStyleSheetTextReturnObject;
        }
        impl Method for TrackComputedStyleUpdates {
            const NAME: &'static str = "CSS.trackComputedStyleUpdates";
            type ReturnObject = TrackComputedStyleUpdatesReturnObject;
        }
        impl Method for TakeComputedStyleUpdates {
            const NAME: &'static str = "CSS.takeComputedStyleUpdates";
            type ReturnObject = TakeComputedStyleUpdatesReturnObject;
        }
        impl Method for SetEffectivePropertyValueForNode {
            const NAME: &'static str = "CSS.setEffectivePropertyValueForNode";
            type ReturnObject = SetEffectivePropertyValueForNodeReturnObject;
        }
        impl Method for SetKeyframeKey {
            const NAME: &'static str = "CSS.setKeyframeKey";
            type ReturnObject = SetKeyframeKeyReturnObject;
        }
        impl Method for SetMediaText {
            const NAME: &'static str = "CSS.setMediaText";
            type ReturnObject = SetMediaTextReturnObject;
        }
        impl Method for SetContainerQueryText {
            const NAME: &'static str = "CSS.setContainerQueryText";
            type ReturnObject = SetContainerQueryTextReturnObject;
        }
        impl Method for SetRuleSelector {
            const NAME: &'static str = "CSS.setRuleSelector";
            type ReturnObject = SetRuleSelectorReturnObject;
        }
        impl Method for SetStyleSheetText {
            const NAME: &'static str = "CSS.setStyleSheetText";
            type ReturnObject = SetStyleSheetTextReturnObject;
        }
        impl Method for SetStyleTexts {
            const NAME: &'static str = "CSS.setStyleTexts";
            type ReturnObject = SetStyleTextsReturnObject;
        }
        impl Method for StartRuleUsageTracking {
            const NAME: &'static str = "CSS.startRuleUsageTracking";
            type ReturnObject = StartRuleUsageTrackingReturnObject;
        }
        impl Method for StopRuleUsageTracking {
            const NAME: &'static str = "CSS.stopRuleUsageTracking";
            type ReturnObject = StopRuleUsageTrackingReturnObject;
        }
        impl Method for TakeCoverageDelta {
            const NAME: &'static str = "CSS.takeCoverageDelta";
            type ReturnObject = TakeCoverageDeltaReturnObject;
        }
        impl Method for SetLocalFontsEnabled {
            const NAME: &'static str = "CSS.setLocalFontsEnabled";
            type ReturnObject = SetLocalFontsEnabledReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FontsUpdatedEvent {
                pub params: FontsUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct FontsUpdatedEventParams {
                #[serde(skip_serializing_if = "Option::is_none")]
                pub font: Option<super::FontFace>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct MediaQueryResultChangedEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct StyleSheetAddedEvent {
                pub params: StyleSheetAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct StyleSheetAddedEventParams {
                pub header: super::CSSStyleSheetHeader,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct StyleSheetChangedEvent {
                pub params: StyleSheetChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct StyleSheetChangedEventParams {
                pub style_sheet_id: super::StyleSheetId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct StyleSheetRemovedEvent {
                pub params: StyleSheetRemovedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct StyleSheetRemovedEventParams {
                pub style_sheet_id: super::StyleSheetId,
            }
        }
    }
    pub mod CacheStorage {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type CacheId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum CachedResponseType {
            Basic,
            Cors,
            Default,
            Error,
            OpaqueResponse,
            OpaqueRedirect,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DataEntry {
            #[serde(default)]
            pub request_url: String,
            #[serde(default)]
            pub request_method: String,
            pub request_headers: Vec<Header>,
            #[serde(default)]
            pub response_time: JsFloat,
            #[serde(default)]
            pub response_status: JsUInt,
            #[serde(default)]
            pub response_status_text: String,
            pub response_Type: CachedResponseType,
            pub response_headers: Vec<Header>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Cache {
            pub cache_id: CacheId,
            #[serde(default)]
            pub security_origin: String,
            #[serde(default)]
            pub cache_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Header {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CachedResponse {
            #[serde(default)]
            pub body: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteCache {
            pub cache_id: CacheId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteEntry {
            pub cache_id: CacheId,
            #[serde(default)]
            pub request: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestCacheNames {
            #[serde(default)]
            pub security_origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestCachedResponse {
            pub cache_id: CacheId,
            #[serde(default)]
            pub request_url: String,
            pub request_headers: Vec<Header>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestEntries {
            pub cache_id: CacheId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub skip_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub page_size: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub path_filter: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteCacheReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteEntryReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestCacheNamesReturnObject {
            pub caches: Vec<Cache>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestCachedResponseReturnObject {
            pub response: CachedResponse,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestEntriesReturnObject {
            pub cache_data_entries: Vec<DataEntry>,
            #[serde(default)]
            pub return_count: JsFloat,
        }
        impl Method for DeleteCache {
            const NAME: &'static str = "CacheStorage.deleteCache";
            type ReturnObject = DeleteCacheReturnObject;
        }
        impl Method for DeleteEntry {
            const NAME: &'static str = "CacheStorage.deleteEntry";
            type ReturnObject = DeleteEntryReturnObject;
        }
        impl Method for RequestCacheNames {
            const NAME: &'static str = "CacheStorage.requestCacheNames";
            type ReturnObject = RequestCacheNamesReturnObject;
        }
        impl Method for RequestCachedResponse {
            const NAME: &'static str = "CacheStorage.requestCachedResponse";
            type ReturnObject = RequestCachedResponseReturnObject;
        }
        impl Method for RequestEntries {
            const NAME: &'static str = "CacheStorage.requestEntries";
            type ReturnObject = RequestEntriesReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod Cast {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Sink {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub session: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub presentation_url: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetSinkToUse {
            #[serde(default)]
            pub sink_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartTabMirroring {
            #[serde(default)]
            pub sink_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopCasting {
            #[serde(default)]
            pub sink_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetSinkToUseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartTabMirroringReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopCastingReturnObject {}
        impl Method for Enable {
            const NAME: &'static str = "Cast.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Cast.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for SetSinkToUse {
            const NAME: &'static str = "Cast.setSinkToUse";
            type ReturnObject = SetSinkToUseReturnObject;
        }
        impl Method for StartTabMirroring {
            const NAME: &'static str = "Cast.startTabMirroring";
            type ReturnObject = StartTabMirroringReturnObject;
        }
        impl Method for StopCasting {
            const NAME: &'static str = "Cast.stopCasting";
            type ReturnObject = StopCastingReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SinksUpdatedEvent {
                pub params: SinksUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct SinksUpdatedEventParams {
                pub sinks: Vec<super::Sink>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct IssueUpdatedEvent {
                pub params: IssueUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct IssueUpdatedEventParams {
                #[serde(default)]
                pub issue_message: String,
            }
        }
    }
    pub mod DOM {
        use super::types::*;
        use super::Page;
        use super::Runtime;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type NodeId = JsUInt;
        pub type BackendNodeId = JsUInt;
        pub type Quad = Vec<JsFloat>;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum PseudoType {
            FirstLine,
            FirstLetter,
            Before,
            After,
            Marker,
            Backdrop,
            Selection,
            TargetText,
            SpellingError,
            GrammarError,
            Highlight,
            FirstLineInherited,
            Scrollbar,
            ScrollbarThumb,
            ScrollbarButton,
            ScrollbarTrack,
            ScrollbarTrackPiece,
            ScrollbarCorner,
            Resizer,
            InputListButton,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum ShadowRootType {
            UserAgent,
            Open,
            Closed,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum CompatibilityMode {
            QuirksMode,
            LimitedQuirksMode,
            NoQuirksMode,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BackendNode {
            #[serde(default)]
            pub node_type: JsUInt,
            #[serde(default)]
            pub node_name: String,
            pub backend_node_id: BackendNodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Node {
            pub node_id: NodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub parent_id: Option<NodeId>,
            pub backend_node_id: BackendNodeId,
            #[serde(default)]
            pub node_type: JsUInt,
            #[serde(default)]
            pub node_name: String,
            #[serde(default)]
            pub local_name: String,
            #[serde(default)]
            pub node_value: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub child_node_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub children: Option<Vec<Node>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub attributes: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub document_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub base_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub public_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub system_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub internal_subset: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub xml_version: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub value: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub pseudo_Type: Option<PseudoType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub shadow_root_Type: Option<ShadowRootType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub frame_id: Option<Page::FrameId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub content_document: Option<Box<Node>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub shadow_roots: Option<Vec<Node>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub template_content: Option<Box<Node>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub pseudo_elements: Option<Vec<Node>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub imported_document: Option<Box<Node>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub distributed_nodes: Option<Vec<BackendNode>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub is_svg: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub compatibility_mode: Option<CompatibilityMode>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RGBA {
            #[serde(default)]
            pub r: JsUInt,
            #[serde(default)]
            pub g: JsUInt,
            #[serde(default)]
            pub b: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub a: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BoxModel {
            pub content: Quad,
            pub padding: Quad,
            pub border: Quad,
            pub margin: Quad,
            #[serde(default)]
            pub width: JsUInt,
            #[serde(default)]
            pub height: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub shape_outside: Option<ShapeOutsideInfo>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ShapeOutsideInfo {
            pub bounds: Quad,
            #[serde(default)]
            pub shape: Vec<Json>,
            #[serde(default)]
            pub margin_shape: Vec<Json>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Rect {
            #[serde(default)]
            pub x: JsFloat,
            #[serde(default)]
            pub y: JsFloat,
            #[serde(default)]
            pub width: JsFloat,
            #[serde(default)]
            pub height: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CSSComputedStyleProperty {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CollectClassNamesFromSubtree {
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CopyTo {
            pub node_id: NodeId,
            pub target_node_id: NodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub insert_before_node_id: Option<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DescribeNode {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub object_id: Option<Runtime::RemoteObjectId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub depth: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub pierce: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ScrollIntoViewIfNeeded {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub object_id: Option<Runtime::RemoteObjectId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub rect: Option<Rect>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DiscardSearchResults {
            #[serde(default)]
            pub search_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Focus {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub object_id: Option<Runtime::RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAttributes {
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBoxModel {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub object_id: Option<Runtime::RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetContentQuads {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub object_id: Option<Runtime::RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetDocument {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub depth: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub pierce: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetFlattenedDocument {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub depth: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub pierce: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetNodesForSubtreeByStyle {
            pub node_id: NodeId,
            pub computed_styles: Vec<CSSComputedStyleProperty>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub pierce: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetNodeForLocation {
            #[serde(default)]
            pub x: JsUInt,
            #[serde(default)]
            pub y: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub include_user_agent_shadow_dom: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub ignore_pointer_events_none: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetOuterHTML {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub object_id: Option<Runtime::RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetRelayoutBoundary {
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetSearchResults {
            #[serde(default)]
            pub search_id: String,
            #[serde(default)]
            pub from_index: JsUInt,
            #[serde(default)]
            pub to_index: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HideHighlight(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightNode(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightRect(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct MarkUndoableState(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct MoveTo {
            pub node_id: NodeId,
            pub target_node_id: NodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub insert_before_node_id: Option<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PerformSearch {
            #[serde(default)]
            pub query: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub include_user_agent_shadow_dom: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PushNodeByPathToFrontend {
            #[serde(default)]
            pub path: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PushNodesByBackendIdsToFrontend {
            pub backend_node_ids: Vec<BackendNodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct QuerySelector {
            pub node_id: NodeId,
            #[serde(default)]
            pub selector: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct QuerySelectorAll {
            pub node_id: NodeId,
            #[serde(default)]
            pub selector: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Redo(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveAttribute {
            pub node_id: NodeId,
            #[serde(default)]
            pub name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveNode {
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestChildNodes {
            pub node_id: NodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub depth: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub pierce: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestNode {
            pub object_id: Runtime::RemoteObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResolveNode {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub object_group: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub execution_context_id: Option<Runtime::ExecutionContextId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAttributeValue {
            pub node_id: NodeId,
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAttributesAsText {
            pub node_id: NodeId,
            #[serde(default)]
            pub text: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetFileInputFiles {
            #[serde(default)]
            pub files: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub object_id: Option<Runtime::RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetNodeStackTracesEnabled {
            #[serde(default)]
            pub enable: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetNodeStackTraces {
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetFileInfo {
            pub object_id: Runtime::RemoteObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInspectedNode {
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetNodeName {
            pub node_id: NodeId,
            #[serde(default)]
            pub name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetNodeValue {
            pub node_id: NodeId,
            #[serde(default)]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetOuterHTML {
            pub node_id: NodeId,
            #[serde(default)]
            pub outer_html: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Undo(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetFrameOwner {
            pub frame_id: Page::FrameId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetContainerForNode {
            pub node_id: NodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub container_name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetQueryingDescendantsForContainer {
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CollectClassNamesFromSubtreeReturnObject {
            pub class_names: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CopyToReturnObject {
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DescribeNodeReturnObject {
            pub node: Node,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ScrollIntoViewIfNeededReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DiscardSearchResultsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FocusReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAttributesReturnObject {
            pub attributes: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBoxModelReturnObject {
            pub model: BoxModel,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetContentQuadsReturnObject {
            pub quads: Vec<Quad>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetDocumentReturnObject {
            pub root: Node,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetFlattenedDocumentReturnObject {
            pub nodes: Vec<Node>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetNodesForSubtreeByStyleReturnObject {
            pub node_ids: Vec<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetNodeForLocationReturnObject {
            pub backend_node_id: BackendNodeId,
            pub frame_id: Page::FrameId,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetOuterHTMLReturnObject {
            #[serde(default)]
            pub outer_html: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetRelayoutBoundaryReturnObject {
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetSearchResultsReturnObject {
            pub node_ids: Vec<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HideHighlightReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightNodeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightRectReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct MarkUndoableStateReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct MoveToReturnObject {
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PerformSearchReturnObject {
            #[serde(default)]
            pub search_id: String,
            #[serde(default)]
            pub result_count: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PushNodeByPathToFrontendReturnObject {
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PushNodesByBackendIdsToFrontendReturnObject {
            pub node_ids: Vec<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct QuerySelectorReturnObject {
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct QuerySelectorAllReturnObject {
            pub node_ids: Vec<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RedoReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveAttributeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveNodeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestChildNodesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestNodeReturnObject {
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResolveNodeReturnObject {
            pub object: Runtime::RemoteObject,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAttributeValueReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAttributesAsTextReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetFileInputFilesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetNodeStackTracesEnabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetNodeStackTracesReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub creation: Option<Runtime::StackTrace>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetFileInfoReturnObject {
            #[serde(default)]
            pub path: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInspectedNodeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetNodeNameReturnObject {
            pub node_id: NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetNodeValueReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetOuterHTMLReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UndoReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetFrameOwnerReturnObject {
            pub backend_node_id: BackendNodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetContainerForNodeReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<NodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetQueryingDescendantsForContainerReturnObject {
            pub node_ids: Vec<NodeId>,
        }
        impl Method for CollectClassNamesFromSubtree {
            const NAME: &'static str = "DOM.collectClassNamesFromSubtree";
            type ReturnObject = CollectClassNamesFromSubtreeReturnObject;
        }
        impl Method for CopyTo {
            const NAME: &'static str = "DOM.copyTo";
            type ReturnObject = CopyToReturnObject;
        }
        impl Method for DescribeNode {
            const NAME: &'static str = "DOM.describeNode";
            type ReturnObject = DescribeNodeReturnObject;
        }
        impl Method for ScrollIntoViewIfNeeded {
            const NAME: &'static str = "DOM.scrollIntoViewIfNeeded";
            type ReturnObject = ScrollIntoViewIfNeededReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "DOM.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for DiscardSearchResults {
            const NAME: &'static str = "DOM.discardSearchResults";
            type ReturnObject = DiscardSearchResultsReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "DOM.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for Focus {
            const NAME: &'static str = "DOM.focus";
            type ReturnObject = FocusReturnObject;
        }
        impl Method for GetAttributes {
            const NAME: &'static str = "DOM.getAttributes";
            type ReturnObject = GetAttributesReturnObject;
        }
        impl Method for GetBoxModel {
            const NAME: &'static str = "DOM.getBoxModel";
            type ReturnObject = GetBoxModelReturnObject;
        }
        impl Method for GetContentQuads {
            const NAME: &'static str = "DOM.getContentQuads";
            type ReturnObject = GetContentQuadsReturnObject;
        }
        impl Method for GetDocument {
            const NAME: &'static str = "DOM.getDocument";
            type ReturnObject = GetDocumentReturnObject;
        }
        impl Method for GetFlattenedDocument {
            const NAME: &'static str = "DOM.getFlattenedDocument";
            type ReturnObject = GetFlattenedDocumentReturnObject;
        }
        impl Method for GetNodesForSubtreeByStyle {
            const NAME: &'static str = "DOM.getNodesForSubtreeByStyle";
            type ReturnObject = GetNodesForSubtreeByStyleReturnObject;
        }
        impl Method for GetNodeForLocation {
            const NAME: &'static str = "DOM.getNodeForLocation";
            type ReturnObject = GetNodeForLocationReturnObject;
        }
        impl Method for GetOuterHTML {
            const NAME: &'static str = "DOM.getOuterHTML";
            type ReturnObject = GetOuterHTMLReturnObject;
        }
        impl Method for GetRelayoutBoundary {
            const NAME: &'static str = "DOM.getRelayoutBoundary";
            type ReturnObject = GetRelayoutBoundaryReturnObject;
        }
        impl Method for GetSearchResults {
            const NAME: &'static str = "DOM.getSearchResults";
            type ReturnObject = GetSearchResultsReturnObject;
        }
        impl Method for HideHighlight {
            const NAME: &'static str = "DOM.hideHighlight";
            type ReturnObject = HideHighlightReturnObject;
        }
        impl Method for HighlightNode {
            const NAME: &'static str = "DOM.highlightNode";
            type ReturnObject = HighlightNodeReturnObject;
        }
        impl Method for HighlightRect {
            const NAME: &'static str = "DOM.highlightRect";
            type ReturnObject = HighlightRectReturnObject;
        }
        impl Method for MarkUndoableState {
            const NAME: &'static str = "DOM.markUndoableState";
            type ReturnObject = MarkUndoableStateReturnObject;
        }
        impl Method for MoveTo {
            const NAME: &'static str = "DOM.moveTo";
            type ReturnObject = MoveToReturnObject;
        }
        impl Method for PerformSearch {
            const NAME: &'static str = "DOM.performSearch";
            type ReturnObject = PerformSearchReturnObject;
        }
        impl Method for PushNodeByPathToFrontend {
            const NAME: &'static str = "DOM.pushNodeByPathToFrontend";
            type ReturnObject = PushNodeByPathToFrontendReturnObject;
        }
        impl Method for PushNodesByBackendIdsToFrontend {
            const NAME: &'static str = "DOM.pushNodesByBackendIdsToFrontend";
            type ReturnObject = PushNodesByBackendIdsToFrontendReturnObject;
        }
        impl Method for QuerySelector {
            const NAME: &'static str = "DOM.querySelector";
            type ReturnObject = QuerySelectorReturnObject;
        }
        impl Method for QuerySelectorAll {
            const NAME: &'static str = "DOM.querySelectorAll";
            type ReturnObject = QuerySelectorAllReturnObject;
        }
        impl Method for Redo {
            const NAME: &'static str = "DOM.redo";
            type ReturnObject = RedoReturnObject;
        }
        impl Method for RemoveAttribute {
            const NAME: &'static str = "DOM.removeAttribute";
            type ReturnObject = RemoveAttributeReturnObject;
        }
        impl Method for RemoveNode {
            const NAME: &'static str = "DOM.removeNode";
            type ReturnObject = RemoveNodeReturnObject;
        }
        impl Method for RequestChildNodes {
            const NAME: &'static str = "DOM.requestChildNodes";
            type ReturnObject = RequestChildNodesReturnObject;
        }
        impl Method for RequestNode {
            const NAME: &'static str = "DOM.requestNode";
            type ReturnObject = RequestNodeReturnObject;
        }
        impl Method for ResolveNode {
            const NAME: &'static str = "DOM.resolveNode";
            type ReturnObject = ResolveNodeReturnObject;
        }
        impl Method for SetAttributeValue {
            const NAME: &'static str = "DOM.setAttributeValue";
            type ReturnObject = SetAttributeValueReturnObject;
        }
        impl Method for SetAttributesAsText {
            const NAME: &'static str = "DOM.setAttributesAsText";
            type ReturnObject = SetAttributesAsTextReturnObject;
        }
        impl Method for SetFileInputFiles {
            const NAME: &'static str = "DOM.setFileInputFiles";
            type ReturnObject = SetFileInputFilesReturnObject;
        }
        impl Method for SetNodeStackTracesEnabled {
            const NAME: &'static str = "DOM.setNodeStackTracesEnabled";
            type ReturnObject = SetNodeStackTracesEnabledReturnObject;
        }
        impl Method for GetNodeStackTraces {
            const NAME: &'static str = "DOM.getNodeStackTraces";
            type ReturnObject = GetNodeStackTracesReturnObject;
        }
        impl Method for GetFileInfo {
            const NAME: &'static str = "DOM.getFileInfo";
            type ReturnObject = GetFileInfoReturnObject;
        }
        impl Method for SetInspectedNode {
            const NAME: &'static str = "DOM.setInspectedNode";
            type ReturnObject = SetInspectedNodeReturnObject;
        }
        impl Method for SetNodeName {
            const NAME: &'static str = "DOM.setNodeName";
            type ReturnObject = SetNodeNameReturnObject;
        }
        impl Method for SetNodeValue {
            const NAME: &'static str = "DOM.setNodeValue";
            type ReturnObject = SetNodeValueReturnObject;
        }
        impl Method for SetOuterHTML {
            const NAME: &'static str = "DOM.setOuterHTML";
            type ReturnObject = SetOuterHTMLReturnObject;
        }
        impl Method for Undo {
            const NAME: &'static str = "DOM.undo";
            type ReturnObject = UndoReturnObject;
        }
        impl Method for GetFrameOwner {
            const NAME: &'static str = "DOM.getFrameOwner";
            type ReturnObject = GetFrameOwnerReturnObject;
        }
        impl Method for GetContainerForNode {
            const NAME: &'static str = "DOM.getContainerForNode";
            type ReturnObject = GetContainerForNodeReturnObject;
        }
        impl Method for GetQueryingDescendantsForContainer {
            const NAME: &'static str = "DOM.getQueryingDescendantsForContainer";
            type ReturnObject = GetQueryingDescendantsForContainerReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AttributeModifiedEvent {
                pub params: AttributeModifiedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AttributeModifiedEventParams {
                pub node_id: super::NodeId,
                #[serde(default)]
                pub name: String,
                #[serde(default)]
                pub value: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AttributeRemovedEvent {
                pub params: AttributeRemovedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AttributeRemovedEventParams {
                pub node_id: super::NodeId,
                #[serde(default)]
                pub name: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct CharacterDataModifiedEvent {
                pub params: CharacterDataModifiedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct CharacterDataModifiedEventParams {
                pub node_id: super::NodeId,
                #[serde(default)]
                pub character_data: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ChildNodeCountUpdatedEvent {
                pub params: ChildNodeCountUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ChildNodeCountUpdatedEventParams {
                pub node_id: super::NodeId,
                #[serde(default)]
                pub child_node_count: JsUInt,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ChildNodeInsertedEvent {
                pub params: ChildNodeInsertedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ChildNodeInsertedEventParams {
                pub parent_node_id: super::NodeId,
                pub previous_node_id: super::NodeId,
                pub node: super::Node,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ChildNodeRemovedEvent {
                pub params: ChildNodeRemovedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ChildNodeRemovedEventParams {
                pub parent_node_id: super::NodeId,
                pub node_id: super::NodeId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DistributedNodesUpdatedEvent {
                pub params: DistributedNodesUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DistributedNodesUpdatedEventParams {
                pub insertion_point_id: super::NodeId,
                pub distributed_nodes: Vec<super::BackendNode>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DocumentUpdatedEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct InlineStyleInvalidatedEvent {
                pub params: InlineStyleInvalidatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct InlineStyleInvalidatedEventParams {
                pub node_ids: Vec<super::NodeId>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PseudoElementAddedEvent {
                pub params: PseudoElementAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct PseudoElementAddedEventParams {
                pub parent_id: super::NodeId,
                pub pseudo_element: super::Node,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PseudoElementRemovedEvent {
                pub params: PseudoElementRemovedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct PseudoElementRemovedEventParams {
                pub parent_id: super::NodeId,
                pub pseudo_element_id: super::NodeId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SetChildNodesEvent {
                pub params: SetChildNodesEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct SetChildNodesEventParams {
                pub parent_id: super::NodeId,
                pub nodes: Vec<super::Node>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ShadowRootPoppedEvent {
                pub params: ShadowRootPoppedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ShadowRootPoppedEventParams {
                pub host_id: super::NodeId,
                pub root_id: super::NodeId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ShadowRootPushedEvent {
                pub params: ShadowRootPushedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ShadowRootPushedEventParams {
                pub host_id: super::NodeId,
                pub root: super::Node,
            }
        }
    }
    pub mod DOMDebugger {
        use super::types::*;
        use super::Debugger;
        use super::Runtime;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum DOMBreakpointType {
            SubtreeModified,
            AttributeModified,
            NodeRemoved,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum CSPViolationType {
            TrustedtypeSinkViolation,
            TrustedtypePolicyViolation,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EventListener {
            #[serde(default)]
            pub Type: String,
            #[serde(default)]
            pub use_capture: bool,
            #[serde(default)]
            pub passive: bool,
            #[serde(default)]
            pub once: bool,
            pub script_id: Runtime::ScriptId,
            #[serde(default)]
            pub line_number: JsUInt,
            #[serde(default)]
            pub column_number: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub handler: Option<Runtime::RemoteObject>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub original_handler: Option<Runtime::RemoteObject>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetEventListeners {
            pub object_id: Runtime::RemoteObjectId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub depth: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub pierce: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveDOMBreakpoint {
            pub node_id: DOM::NodeId,
            pub Type: DOMBreakpointType,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveEventListenerBreakpoint {
            #[serde(default)]
            pub event_name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub target_name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveInstrumentationBreakpoint {
            #[serde(default)]
            pub event_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveXHRBreakpoint {
            #[serde(default)]
            pub url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBreakOnCSPViolation {
            pub violation_Types: Vec<CSPViolationType>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDOMBreakpoint {
            pub node_id: DOM::NodeId,
            pub Type: DOMBreakpointType,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetEventListenerBreakpoint {
            #[serde(default)]
            pub event_name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub target_name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInstrumentationBreakpoint {
            #[serde(default)]
            pub event_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetXHRBreakpoint {
            #[serde(default)]
            pub url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetEventListenersReturnObject {
            pub listeners: Vec<EventListener>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveDOMBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveEventListenerBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveInstrumentationBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveXHRBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBreakOnCSPViolationReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDOMBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetEventListenerBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInstrumentationBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetXHRBreakpointReturnObject {}
        impl Method for GetEventListeners {
            const NAME: &'static str = "DOMDebugger.getEventListeners";
            type ReturnObject = GetEventListenersReturnObject;
        }
        impl Method for RemoveDOMBreakpoint {
            const NAME: &'static str = "DOMDebugger.removeDOMBreakpoint";
            type ReturnObject = RemoveDOMBreakpointReturnObject;
        }
        impl Method for RemoveEventListenerBreakpoint {
            const NAME: &'static str = "DOMDebugger.removeEventListenerBreakpoint";
            type ReturnObject = RemoveEventListenerBreakpointReturnObject;
        }
        impl Method for RemoveInstrumentationBreakpoint {
            const NAME: &'static str = "DOMDebugger.removeInstrumentationBreakpoint";
            type ReturnObject = RemoveInstrumentationBreakpointReturnObject;
        }
        impl Method for RemoveXHRBreakpoint {
            const NAME: &'static str = "DOMDebugger.removeXHRBreakpoint";
            type ReturnObject = RemoveXHRBreakpointReturnObject;
        }
        impl Method for SetBreakOnCSPViolation {
            const NAME: &'static str = "DOMDebugger.setBreakOnCSPViolation";
            type ReturnObject = SetBreakOnCSPViolationReturnObject;
        }
        impl Method for SetDOMBreakpoint {
            const NAME: &'static str = "DOMDebugger.setDOMBreakpoint";
            type ReturnObject = SetDOMBreakpointReturnObject;
        }
        impl Method for SetEventListenerBreakpoint {
            const NAME: &'static str = "DOMDebugger.setEventListenerBreakpoint";
            type ReturnObject = SetEventListenerBreakpointReturnObject;
        }
        impl Method for SetInstrumentationBreakpoint {
            const NAME: &'static str = "DOMDebugger.setInstrumentationBreakpoint";
            type ReturnObject = SetInstrumentationBreakpointReturnObject;
        }
        impl Method for SetXHRBreakpoint {
            const NAME: &'static str = "DOMDebugger.setXHRBreakpoint";
            type ReturnObject = SetXHRBreakpointReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod EventBreakpoints {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInstrumentationBreakpoint {
            #[serde(default)]
            pub event_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveInstrumentationBreakpoint {
            #[serde(default)]
            pub event_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInstrumentationBreakpointReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveInstrumentationBreakpointReturnObject {}
        impl Method for SetInstrumentationBreakpoint {
            const NAME: &'static str = "EventBreakpoints.setInstrumentationBreakpoint";
            type ReturnObject = SetInstrumentationBreakpointReturnObject;
        }
        impl Method for RemoveInstrumentationBreakpoint {
            const NAME: &'static str = "EventBreakpoints.removeInstrumentationBreakpoint";
            type ReturnObject = RemoveInstrumentationBreakpointReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod DOMSnapshot {
        use super::types::*;
        use super::DOMDebugger;
        use super::Page;
        use super::CSS;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type StringIndex = JsUInt;
        pub type ArrayOfStrings = Vec<StringIndex>;
        pub type Rectangle = Vec<JsFloat>;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DOMNode {
            #[serde(default)]
            pub node_type: JsUInt,
            #[serde(default)]
            pub node_name: String,
            #[serde(default)]
            pub node_value: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub text_value: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub input_value: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub input_checked: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub option_selected: Option<bool>,
            pub backend_node_id: DOM::BackendNodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub child_node_indexes: Option<Vec<JsUInt>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub attributes: Option<Vec<NameValue>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub pseudo_element_indexes: Option<Vec<JsUInt>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub layout_node_index: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub document_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub base_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub content_language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub document_encoding: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub public_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub system_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub frame_id: Option<Page::FrameId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub content_document_index: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub pseudo_Type: Option<DOM::PseudoType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub shadow_root_Type: Option<DOM::ShadowRootType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub is_clickable: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub event_listeners: Option<Vec<DOMDebugger::EventListener>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub current_source_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub origin_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub scroll_offset_x: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub scroll_offset_y: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct InlineTextBox {
            pub bounding_box: DOM::Rect,
            #[serde(default)]
            pub start_character_index: JsUInt,
            #[serde(default)]
            pub num_characters: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LayoutTreeNode {
            #[serde(default)]
            pub dom_node_index: JsUInt,
            pub bounding_box: DOM::Rect,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub layout_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub inline_text_nodes: Option<Vec<InlineTextBox>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub style_index: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub paint_order: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub is_stacking_context: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ComputedStyle {
            pub properties: Vec<NameValue>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct NameValue {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RareStringData {
            #[serde(default)]
            pub index: Vec<JsUInt>,
            pub value: Vec<StringIndex>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RareBooleanData {
            #[serde(default)]
            pub index: Vec<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RareIntegerData {
            #[serde(default)]
            pub index: Vec<JsUInt>,
            #[serde(default)]
            pub value: Vec<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DocumentSnapshot {
            pub document_url: StringIndex,
            pub title: StringIndex,
            pub base_url: StringIndex,
            pub content_language: StringIndex,
            pub encoding_name: StringIndex,
            pub public_id: StringIndex,
            pub system_id: StringIndex,
            pub frame_id: StringIndex,
            pub nodes: NodeTreeSnapshot,
            pub layout: LayoutTreeSnapshot,
            pub text_boxes: TextBoxSnapshot,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub scroll_offset_x: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub scroll_offset_y: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub content_width: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub content_height: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct NodeTreeSnapshot {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub parent_index: Option<Vec<JsUInt>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub node_type: Option<Vec<JsUInt>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub shadow_root_Type: Option<RareStringData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_name: Option<Vec<StringIndex>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_value: Option<Vec<StringIndex>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<Vec<DOM::BackendNodeId>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub attributes: Option<Vec<ArrayOfStrings>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub text_value: Option<RareStringData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub input_value: Option<RareStringData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub input_checked: Option<RareBooleanData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub option_selected: Option<RareBooleanData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub content_document_index: Option<RareIntegerData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub pseudo_Type: Option<RareStringData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub is_clickable: Option<RareBooleanData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub current_source_url: Option<RareStringData>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub origin_url: Option<RareStringData>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LayoutTreeSnapshot {
            #[serde(default)]
            pub node_index: Vec<JsUInt>,
            pub styles: Vec<ArrayOfStrings>,
            pub bounds: Vec<Rectangle>,
            pub text: Vec<StringIndex>,
            pub stacking_contexts: RareBooleanData,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub paint_orders: Option<Vec<JsUInt>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub offset_rects: Option<Vec<Rectangle>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub scroll_rects: Option<Vec<Rectangle>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub client_rects: Option<Vec<Rectangle>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub blended_background_colors: Option<Vec<StringIndex>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub text_color_opacities: Option<Vec<JsFloat>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TextBoxSnapshot {
            #[serde(default)]
            pub layout_index: Vec<JsUInt>,
            pub bounds: Vec<Rectangle>,
            #[serde(default)]
            pub start: Vec<JsUInt>,
            #[serde(default)]
            pub length: Vec<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetSnapshot {
            #[serde(default)]
            pub computed_style_whitelist: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub include_event_listeners: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub include_paint_order: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub include_user_agent_shadow_tree: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CaptureSnapshot {
            #[serde(default)]
            pub computed_styles: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub include_paint_order: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub include_dom_rects: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub include_blended_background_colors: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub include_text_color_opacities: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetSnapshotReturnObject {
            pub dom_nodes: Vec<DOMNode>,
            pub layout_tree_nodes: Vec<LayoutTreeNode>,
            pub computed_styles: Vec<ComputedStyle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CaptureSnapshotReturnObject {
            pub documents: Vec<DocumentSnapshot>,
            pub strings: Vec<String>,
        }
        impl Method for Disable {
            const NAME: &'static str = "DOMSnapshot.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "DOMSnapshot.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetSnapshot {
            const NAME: &'static str = "DOMSnapshot.getSnapshot";
            type ReturnObject = GetSnapshotReturnObject;
        }
        impl Method for CaptureSnapshot {
            const NAME: &'static str = "DOMSnapshot.captureSnapshot";
            type ReturnObject = CaptureSnapshotReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod DOMStorage {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type Item = Vec<String>;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StorageId {
            #[serde(default)]
            pub security_origin: String,
            #[serde(default)]
            pub is_local_storage: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Clear {
            pub storage_id: StorageId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetDOMStorageItems {
            pub storage_id: StorageId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveDOMStorageItem {
            pub storage_id: StorageId,
            #[serde(default)]
            pub key: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDOMStorageItem {
            pub storage_id: StorageId,
            #[serde(default)]
            pub key: String,
            #[serde(default)]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetDOMStorageItemsReturnObject {
            pub entries: Vec<Item>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveDOMStorageItemReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDOMStorageItemReturnObject {}
        impl Method for Clear {
            const NAME: &'static str = "DOMStorage.clear";
            type ReturnObject = ClearReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "DOMStorage.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "DOMStorage.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetDOMStorageItems {
            const NAME: &'static str = "DOMStorage.getDOMStorageItems";
            type ReturnObject = GetDOMStorageItemsReturnObject;
        }
        impl Method for RemoveDOMStorageItem {
            const NAME: &'static str = "DOMStorage.removeDOMStorageItem";
            type ReturnObject = RemoveDOMStorageItemReturnObject;
        }
        impl Method for SetDOMStorageItem {
            const NAME: &'static str = "DOMStorage.setDOMStorageItem";
            type ReturnObject = SetDOMStorageItemReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DomStorageItemAddedEvent {
                pub params: DomStorageItemAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DomStorageItemAddedEventParams {
                pub storage_id: super::StorageId,
                #[serde(default)]
                pub key: String,
                #[serde(default)]
                pub new_value: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DomStorageItemRemovedEvent {
                pub params: DomStorageItemRemovedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DomStorageItemRemovedEventParams {
                pub storage_id: super::StorageId,
                #[serde(default)]
                pub key: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DomStorageItemUpdatedEvent {
                pub params: DomStorageItemUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DomStorageItemUpdatedEventParams {
                pub storage_id: super::StorageId,
                #[serde(default)]
                pub key: String,
                #[serde(default)]
                pub old_value: String,
                #[serde(default)]
                pub new_value: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DomStorageItemsClearedEvent {
                pub params: DomStorageItemsClearedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DomStorageItemsClearedEventParams {
                pub storage_id: super::StorageId,
            }
        }
    }
    pub mod Database {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type DatabaseId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Database {
            pub id: DatabaseId,
            #[serde(default)]
            pub domain: String,
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub version: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Error {
            #[serde(default)]
            pub message: String,
            #[serde(default)]
            pub code: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ExecuteSQL {
            pub database_id: DatabaseId,
            #[serde(default)]
            pub query: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetDatabaseTableNames {
            pub database_id: DatabaseId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ExecuteSQLReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub column_names: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub values: Option<Vec<Json>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub sql_error: Option<Error>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetDatabaseTableNamesReturnObject {
            pub table_names: Vec<String>,
        }
        impl Method for Disable {
            const NAME: &'static str = "Database.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Database.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for ExecuteSQL {
            const NAME: &'static str = "Database.executeSQL";
            type ReturnObject = ExecuteSQLReturnObject;
        }
        impl Method for GetDatabaseTableNames {
            const NAME: &'static str = "Database.getDatabaseTableNames";
            type ReturnObject = GetDatabaseTableNamesReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AddDatabaseEvent {
                pub params: AddDatabaseEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AddDatabaseEventParams {
                pub database: super::Database,
            }
        }
    }
    pub mod DeviceOrientation {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceOrientationOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDeviceOrientationOverride {
            #[serde(default)]
            pub alpha: JsFloat,
            #[serde(default)]
            pub beta: JsFloat,
            #[serde(default)]
            pub gamma: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceOrientationOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDeviceOrientationOverrideReturnObject {}
        impl Method for ClearDeviceOrientationOverride {
            const NAME: &'static str = "DeviceOrientation.clearDeviceOrientationOverride";
            type ReturnObject = ClearDeviceOrientationOverrideReturnObject;
        }
        impl Method for SetDeviceOrientationOverride {
            const NAME: &'static str = "DeviceOrientation.setDeviceOrientationOverride";
            type ReturnObject = SetDeviceOrientationOverrideReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod Emulation {
        use super::types::*;
        use super::Network;
        use super::Page;
        use super::Runtime;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ScreenOrientationType {
            PortraitPrimary,
            PortraitSecondary,
            LandscapePrimary,
            LandscapeSecondary,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum DisplayFeatureOrientation {
            Vertical,
            Horizontal,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum VirtualTimePolicy {
            Advance,
            Pause,
            PauseIfNetworkFetchesPending,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum DisabledImageType {
            Avif,
            Jxl,
            Webp,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum SetEmitTouchEventsForMouseConfigurationOption {
            Mobile,
            Desktop,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum SetEmulatedVisionDeficiencyTypeOption {
            None,
            Achromatopsia,
            BlurredVision,
            Deuteranopia,
            Protanopia,
            Tritanopia,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ScreenOrientation {
            pub Type: ScreenOrientationType,
            #[serde(default)]
            pub angle: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisplayFeature {
            pub orientation: DisplayFeatureOrientation,
            #[serde(default)]
            pub offset: JsUInt,
            #[serde(default)]
            pub mask_length: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct MediaFeature {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UserAgentBrandVersion {
            #[serde(default)]
            pub brand: String,
            #[serde(default)]
            pub version: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UserAgentMetadata {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub brands: Option<Vec<UserAgentBrandVersion>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub full_version_list: Option<Vec<UserAgentBrandVersion>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub full_version: Option<String>,
            #[serde(default)]
            pub platform: String,
            #[serde(default)]
            pub platform_version: String,
            #[serde(default)]
            pub architecture: String,
            #[serde(default)]
            pub model: String,
            #[serde(default)]
            pub mobile: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CanEmulate(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceMetricsOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearGeolocationOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResetPageScaleFactor(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetFocusEmulationEnabled {
            #[serde(default)]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAutoDarkModeOverride {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub enabled: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCPUThrottlingRate {
            #[serde(default)]
            pub rate: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDefaultBackgroundColorOverride {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDeviceMetricsOverride {
            #[serde(default)]
            pub width: JsUInt,
            #[serde(default)]
            pub height: JsUInt,
            #[serde(default)]
            pub device_scale_factor: JsFloat,
            #[serde(default)]
            pub mobile: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub scale: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub screen_width: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub screen_height: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub position_x: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub position_y: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub dont_set_visible_size: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub screen_orientation: Option<ScreenOrientation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub viewport: Option<Page::Viewport>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub display_feature: Option<DisplayFeature>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetScrollbarsHidden {
            #[serde(default)]
            pub hidden: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDocumentCookieDisabled {
            #[serde(default)]
            pub disabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetEmitTouchEventsForMouse {
            #[serde(default)]
            pub enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub configuration: Option<SetEmitTouchEventsForMouseConfigurationOption>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetEmulatedMedia {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub media: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub features: Option<Vec<MediaFeature>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetEmulatedVisionDeficiency {
            pub Type: SetEmulatedVisionDeficiencyTypeOption,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetGeolocationOverride {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub latitude: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub longitude: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub accuracy: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetIdleOverride {
            #[serde(default)]
            pub is_user_active: bool,
            #[serde(default)]
            pub is_screen_unlocked: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearIdleOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetNavigatorOverrides {
            #[serde(default)]
            pub platform: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPageScaleFactor {
            #[serde(default)]
            pub page_scale_factor: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetScriptExecutionDisabled {
            #[serde(default)]
            pub value: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetTouchEmulationEnabled {
            #[serde(default)]
            pub enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub max_touch_points: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetVirtualTimePolicy {
            pub policy: VirtualTimePolicy,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub budget: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub max_virtual_time_task_starvation_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub wait_for_navigation: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub initial_virtual_time: Option<Network::TimeSinceEpoch>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetLocaleOverride {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub locale: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetTimezoneOverride {
            #[serde(default)]
            pub timezone_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetVisibleSize {
            #[serde(default)]
            pub width: JsUInt,
            #[serde(default)]
            pub height: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDisabledImageTypes {
            pub image_Types: Vec<DisabledImageType>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetUserAgentOverride {
            #[serde(default)]
            pub user_agent: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub accept_language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub platform: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub user_agent_metadata: Option<UserAgentMetadata>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CanEmulateReturnObject {
            #[serde(default)]
            pub result: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceMetricsOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearGeolocationOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResetPageScaleFactorReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetFocusEmulationEnabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAutoDarkModeOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCPUThrottlingRateReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDefaultBackgroundColorOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDeviceMetricsOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetScrollbarsHiddenReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDocumentCookieDisabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetEmitTouchEventsForMouseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetEmulatedMediaReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetEmulatedVisionDeficiencyReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetGeolocationOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetIdleOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearIdleOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetNavigatorOverridesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPageScaleFactorReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetScriptExecutionDisabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetTouchEmulationEnabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetVirtualTimePolicyReturnObject {
            #[serde(default)]
            pub virtual_time_ticks_base: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetLocaleOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetTimezoneOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetVisibleSizeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDisabledImageTypesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetUserAgentOverrideReturnObject {}
        impl Method for CanEmulate {
            const NAME: &'static str = "Emulation.canEmulate";
            type ReturnObject = CanEmulateReturnObject;
        }
        impl Method for ClearDeviceMetricsOverride {
            const NAME: &'static str = "Emulation.clearDeviceMetricsOverride";
            type ReturnObject = ClearDeviceMetricsOverrideReturnObject;
        }
        impl Method for ClearGeolocationOverride {
            const NAME: &'static str = "Emulation.clearGeolocationOverride";
            type ReturnObject = ClearGeolocationOverrideReturnObject;
        }
        impl Method for ResetPageScaleFactor {
            const NAME: &'static str = "Emulation.resetPageScaleFactor";
            type ReturnObject = ResetPageScaleFactorReturnObject;
        }
        impl Method for SetFocusEmulationEnabled {
            const NAME: &'static str = "Emulation.setFocusEmulationEnabled";
            type ReturnObject = SetFocusEmulationEnabledReturnObject;
        }
        impl Method for SetAutoDarkModeOverride {
            const NAME: &'static str = "Emulation.setAutoDarkModeOverride";
            type ReturnObject = SetAutoDarkModeOverrideReturnObject;
        }
        impl Method for SetCPUThrottlingRate {
            const NAME: &'static str = "Emulation.setCPUThrottlingRate";
            type ReturnObject = SetCPUThrottlingRateReturnObject;
        }
        impl Method for SetDefaultBackgroundColorOverride {
            const NAME: &'static str = "Emulation.setDefaultBackgroundColorOverride";
            type ReturnObject = SetDefaultBackgroundColorOverrideReturnObject;
        }
        impl Method for SetDeviceMetricsOverride {
            const NAME: &'static str = "Emulation.setDeviceMetricsOverride";
            type ReturnObject = SetDeviceMetricsOverrideReturnObject;
        }
        impl Method for SetScrollbarsHidden {
            const NAME: &'static str = "Emulation.setScrollbarsHidden";
            type ReturnObject = SetScrollbarsHiddenReturnObject;
        }
        impl Method for SetDocumentCookieDisabled {
            const NAME: &'static str = "Emulation.setDocumentCookieDisabled";
            type ReturnObject = SetDocumentCookieDisabledReturnObject;
        }
        impl Method for SetEmitTouchEventsForMouse {
            const NAME: &'static str = "Emulation.setEmitTouchEventsForMouse";
            type ReturnObject = SetEmitTouchEventsForMouseReturnObject;
        }
        impl Method for SetEmulatedMedia {
            const NAME: &'static str = "Emulation.setEmulatedMedia";
            type ReturnObject = SetEmulatedMediaReturnObject;
        }
        impl Method for SetEmulatedVisionDeficiency {
            const NAME: &'static str = "Emulation.setEmulatedVisionDeficiency";
            type ReturnObject = SetEmulatedVisionDeficiencyReturnObject;
        }
        impl Method for SetGeolocationOverride {
            const NAME: &'static str = "Emulation.setGeolocationOverride";
            type ReturnObject = SetGeolocationOverrideReturnObject;
        }
        impl Method for SetIdleOverride {
            const NAME: &'static str = "Emulation.setIdleOverride";
            type ReturnObject = SetIdleOverrideReturnObject;
        }
        impl Method for ClearIdleOverride {
            const NAME: &'static str = "Emulation.clearIdleOverride";
            type ReturnObject = ClearIdleOverrideReturnObject;
        }
        impl Method for SetNavigatorOverrides {
            const NAME: &'static str = "Emulation.setNavigatorOverrides";
            type ReturnObject = SetNavigatorOverridesReturnObject;
        }
        impl Method for SetPageScaleFactor {
            const NAME: &'static str = "Emulation.setPageScaleFactor";
            type ReturnObject = SetPageScaleFactorReturnObject;
        }
        impl Method for SetScriptExecutionDisabled {
            const NAME: &'static str = "Emulation.setScriptExecutionDisabled";
            type ReturnObject = SetScriptExecutionDisabledReturnObject;
        }
        impl Method for SetTouchEmulationEnabled {
            const NAME: &'static str = "Emulation.setTouchEmulationEnabled";
            type ReturnObject = SetTouchEmulationEnabledReturnObject;
        }
        impl Method for SetVirtualTimePolicy {
            const NAME: &'static str = "Emulation.setVirtualTimePolicy";
            type ReturnObject = SetVirtualTimePolicyReturnObject;
        }
        impl Method for SetLocaleOverride {
            const NAME: &'static str = "Emulation.setLocaleOverride";
            type ReturnObject = SetLocaleOverrideReturnObject;
        }
        impl Method for SetTimezoneOverride {
            const NAME: &'static str = "Emulation.setTimezoneOverride";
            type ReturnObject = SetTimezoneOverrideReturnObject;
        }
        impl Method for SetVisibleSize {
            const NAME: &'static str = "Emulation.setVisibleSize";
            type ReturnObject = SetVisibleSizeReturnObject;
        }
        impl Method for SetDisabledImageTypes {
            const NAME: &'static str = "Emulation.setDisabledImageTypes";
            type ReturnObject = SetDisabledImageTypesReturnObject;
        }
        impl Method for SetUserAgentOverride {
            const NAME: &'static str = "Emulation.setUserAgentOverride";
            type ReturnObject = SetUserAgentOverrideReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct VirtualTimeBudgetExpiredEvent(pub Option<serde_json::Value>);
        }
    }
    pub mod HeadlessExperimental {
        use super::types::*;
        use super::Page;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ScreenshotParamsFormat {
            Jpeg,
            Png,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ScreenshotParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub format: Option<ScreenshotParamsFormat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub quality: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BeginFrame {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub frame_time_ticks: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub interval: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub no_display_updates: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub screenshot: Option<ScreenshotParams>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BeginFrameReturnObject {
            #[serde(default)]
            pub has_damage: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub screenshot_data: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        impl Method for BeginFrame {
            const NAME: &'static str = "HeadlessExperimental.beginFrame";
            type ReturnObject = BeginFrameReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "HeadlessExperimental.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "HeadlessExperimental.enable";
            type ReturnObject = EnableReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NeedsBeginFramesChangedEvent {
                pub params: NeedsBeginFramesChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct NeedsBeginFramesChangedEventParams {
                #[serde(default)]
                pub needs_begin_frames: bool,
            }
        }
    }
    pub mod IO {
        use super::types::*;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type StreamHandle = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Close {
            pub handle: StreamHandle,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Read {
            pub handle: StreamHandle,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub offset: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub size: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResolveBlob {
            pub object_id: Runtime::RemoteObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CloseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReadReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub base_64_encoded: Option<bool>,
            #[serde(default)]
            pub data: String,
            #[serde(default)]
            pub eof: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResolveBlobReturnObject {
            #[serde(default)]
            pub uuid: String,
        }
        impl Method for Close {
            const NAME: &'static str = "IO.close";
            type ReturnObject = CloseReturnObject;
        }
        impl Method for Read {
            const NAME: &'static str = "IO.read";
            type ReturnObject = ReadReturnObject;
        }
        impl Method for ResolveBlob {
            const NAME: &'static str = "IO.resolveBlob";
            type ReturnObject = ResolveBlobReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod IndexedDB {
        use super::types::*;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum KeyType {
            Number,
            String,
            Date,
            Array,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum KeyPathType {
            Null,
            String,
            Array,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DatabaseWithObjectStores {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub version: JsFloat,
            pub object_stores: Vec<ObjectStore>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ObjectStore {
            #[serde(default)]
            pub name: String,
            pub key_path: KeyPath,
            #[serde(default)]
            pub auto_increment: bool,
            pub indexes: Vec<ObjectStoreIndex>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ObjectStoreIndex {
            #[serde(default)]
            pub name: String,
            pub key_path: KeyPath,
            #[serde(default)]
            pub unique: bool,
            #[serde(default)]
            pub multi_entry: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Key {
            pub Type: KeyType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub number: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub string: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub date: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub array: Option<Vec<Key>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct KeyRange {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub lower: Option<Key>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub upper: Option<Key>,
            #[serde(default)]
            pub lower_open: bool,
            #[serde(default)]
            pub upper_open: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DataEntry {
            pub key: Runtime::RemoteObject,
            pub primary_key: Runtime::RemoteObject,
            pub value: Runtime::RemoteObject,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct KeyPath {
            pub Type: KeyPathType,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub string: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub array: Option<Vec<String>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearObjectStore {
            #[serde(default)]
            pub security_origin: String,
            #[serde(default)]
            pub database_name: String,
            #[serde(default)]
            pub object_store_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteDatabase {
            #[serde(default)]
            pub security_origin: String,
            #[serde(default)]
            pub database_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteObjectStoreEntries {
            #[serde(default)]
            pub security_origin: String,
            #[serde(default)]
            pub database_name: String,
            #[serde(default)]
            pub object_store_name: String,
            pub key_range: KeyRange,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestData {
            #[serde(default)]
            pub security_origin: String,
            #[serde(default)]
            pub database_name: String,
            #[serde(default)]
            pub object_store_name: String,
            #[serde(default)]
            pub index_name: String,
            #[serde(default)]
            pub skip_count: JsUInt,
            #[serde(default)]
            pub page_size: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub key_range: Option<KeyRange>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetMetadata {
            #[serde(default)]
            pub security_origin: String,
            #[serde(default)]
            pub database_name: String,
            #[serde(default)]
            pub object_store_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestDatabase {
            #[serde(default)]
            pub security_origin: String,
            #[serde(default)]
            pub database_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestDatabaseNames {
            #[serde(default)]
            pub security_origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearObjectStoreReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteDatabaseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteObjectStoreEntriesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestDataReturnObject {
            pub object_store_data_entries: Vec<DataEntry>,
            #[serde(default)]
            pub has_more: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetMetadataReturnObject {
            #[serde(default)]
            pub entries_count: JsFloat,
            #[serde(default)]
            pub key_generator_value: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestDatabaseReturnObject {
            pub database_with_object_stores: DatabaseWithObjectStores,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestDatabaseNamesReturnObject {
            pub database_names: Vec<String>,
        }
        impl Method for ClearObjectStore {
            const NAME: &'static str = "IndexedDB.clearObjectStore";
            type ReturnObject = ClearObjectStoreReturnObject;
        }
        impl Method for DeleteDatabase {
            const NAME: &'static str = "IndexedDB.deleteDatabase";
            type ReturnObject = DeleteDatabaseReturnObject;
        }
        impl Method for DeleteObjectStoreEntries {
            const NAME: &'static str = "IndexedDB.deleteObjectStoreEntries";
            type ReturnObject = DeleteObjectStoreEntriesReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "IndexedDB.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "IndexedDB.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for RequestData {
            const NAME: &'static str = "IndexedDB.requestData";
            type ReturnObject = RequestDataReturnObject;
        }
        impl Method for GetMetadata {
            const NAME: &'static str = "IndexedDB.getMetadata";
            type ReturnObject = GetMetadataReturnObject;
        }
        impl Method for RequestDatabase {
            const NAME: &'static str = "IndexedDB.requestDatabase";
            type ReturnObject = RequestDatabaseReturnObject;
        }
        impl Method for RequestDatabaseNames {
            const NAME: &'static str = "IndexedDB.requestDatabaseNames";
            type ReturnObject = RequestDatabaseNamesReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod Input {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type TimeSinceEpoch = JsFloat;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum GestureSourceType {
            Default,
            Touch,
            Mouse,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum MouseButton {
            None,
            Left,
            Middle,
            Right,
            Back,
            Forward,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum DispatchDragEventTypeOption {
            DragEnter,
            DragOver,
            Drop,
            DragCancel,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum DispatchKeyEventTypeOption {
            KeyDown,
            KeyUp,
            RawKeyDown,
            Char,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum DispatchMouseEventTypeOption {
            MousePressed,
            MouseReleased,
            MouseMoved,
            MouseWheel,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum DispatchMouseEventPointer_TypeOption {
            Mouse,
            Pen,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum DispatchTouchEventTypeOption {
            TouchStart,
            TouchEnd,
            TouchMove,
            TouchCancel,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum EmulateTouchFromMouseEventTypeOption {
            MousePressed,
            MouseReleased,
            MouseMoved,
            MouseWheel,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TouchPoint {
            #[serde(default)]
            pub x: JsFloat,
            #[serde(default)]
            pub y: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub radius_x: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub radius_y: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub rotation_angle: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub force: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub tangential_pressure: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub tilt_x: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub tilt_y: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub twist: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub id: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DragDataItem {
            #[serde(default)]
            pub mime_type: String,
            #[serde(default)]
            pub data: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub base_url: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DragData {
            pub items: Vec<DragDataItem>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub files: Option<Vec<String>>,
            #[serde(default)]
            pub drag_operations_mask: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchDragEvent {
            pub Type: DispatchDragEventTypeOption,
            #[serde(default)]
            pub x: JsFloat,
            #[serde(default)]
            pub y: JsFloat,
            pub data: DragData,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub modifiers: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchKeyEvent {
            pub Type: DispatchKeyEventTypeOption,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub modifiers: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub timestamp: Option<TimeSinceEpoch>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub unmodified_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub key_identifier: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub code: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub key: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub windows_virtual_key_code: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub native_virtual_key_code: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub auto_repeat: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub is_keypad: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub is_system_key: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub location: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub commands: Option<Vec<String>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct InsertText {
            #[serde(default)]
            pub text: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ImeSetComposition {
            #[serde(default)]
            pub text: String,
            #[serde(default)]
            pub selection_start: JsUInt,
            #[serde(default)]
            pub selection_end: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub replacement_start: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub replacement_end: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchMouseEvent {
            pub Type: DispatchMouseEventTypeOption,
            #[serde(default)]
            pub x: JsFloat,
            #[serde(default)]
            pub y: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub modifiers: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub timestamp: Option<TimeSinceEpoch>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub button: Option<MouseButton>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub buttons: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub click_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub force: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub tangential_pressure: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub tilt_x: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub tilt_y: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub twist: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub delta_x: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub delta_y: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub pointer_Type: Option<DispatchMouseEventPointer_TypeOption>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchTouchEvent {
            pub Type: DispatchTouchEventTypeOption,
            pub touch_points: Vec<TouchPoint>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub modifiers: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub timestamp: Option<TimeSinceEpoch>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EmulateTouchFromMouseEvent {
            pub Type: EmulateTouchFromMouseEventTypeOption,
            #[serde(default)]
            pub x: JsUInt,
            #[serde(default)]
            pub y: JsUInt,
            pub button: MouseButton,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub timestamp: Option<TimeSinceEpoch>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub delta_x: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub delta_y: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub modifiers: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub click_count: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetIgnoreInputEvents {
            #[serde(default)]
            pub ignore: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInterceptDrags {
            #[serde(default)]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SynthesizePinchGesture {
            #[serde(default)]
            pub x: JsFloat,
            #[serde(default)]
            pub y: JsFloat,
            #[serde(default)]
            pub scale_factor: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub relative_speed: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub gesture_source_Type: Option<GestureSourceType>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SynthesizeScrollGesture {
            #[serde(default)]
            pub x: JsFloat,
            #[serde(default)]
            pub y: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub x_distance: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub y_distance: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub x_overscroll: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub y_overscroll: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub prevent_fling: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub speed: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub gesture_source_Type: Option<GestureSourceType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub repeat_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub repeat_delay_ms: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub interaction_marker_name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SynthesizeTapGesture {
            #[serde(default)]
            pub x: JsFloat,
            #[serde(default)]
            pub y: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub duration: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub tap_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub gesture_source_Type: Option<GestureSourceType>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchDragEventReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchKeyEventReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct InsertTextReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ImeSetCompositionReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchMouseEventReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchTouchEventReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EmulateTouchFromMouseEventReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetIgnoreInputEventsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInterceptDragsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SynthesizePinchGestureReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SynthesizeScrollGestureReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SynthesizeTapGestureReturnObject {}
        impl Method for DispatchDragEvent {
            const NAME: &'static str = "Input.dispatchDragEvent";
            type ReturnObject = DispatchDragEventReturnObject;
        }
        impl Method for DispatchKeyEvent {
            const NAME: &'static str = "Input.dispatchKeyEvent";
            type ReturnObject = DispatchKeyEventReturnObject;
        }
        impl Method for InsertText {
            const NAME: &'static str = "Input.insertText";
            type ReturnObject = InsertTextReturnObject;
        }
        impl Method for ImeSetComposition {
            const NAME: &'static str = "Input.imeSetComposition";
            type ReturnObject = ImeSetCompositionReturnObject;
        }
        impl Method for DispatchMouseEvent {
            const NAME: &'static str = "Input.dispatchMouseEvent";
            type ReturnObject = DispatchMouseEventReturnObject;
        }
        impl Method for DispatchTouchEvent {
            const NAME: &'static str = "Input.dispatchTouchEvent";
            type ReturnObject = DispatchTouchEventReturnObject;
        }
        impl Method for EmulateTouchFromMouseEvent {
            const NAME: &'static str = "Input.emulateTouchFromMouseEvent";
            type ReturnObject = EmulateTouchFromMouseEventReturnObject;
        }
        impl Method for SetIgnoreInputEvents {
            const NAME: &'static str = "Input.setIgnoreInputEvents";
            type ReturnObject = SetIgnoreInputEventsReturnObject;
        }
        impl Method for SetInterceptDrags {
            const NAME: &'static str = "Input.setInterceptDrags";
            type ReturnObject = SetInterceptDragsReturnObject;
        }
        impl Method for SynthesizePinchGesture {
            const NAME: &'static str = "Input.synthesizePinchGesture";
            type ReturnObject = SynthesizePinchGestureReturnObject;
        }
        impl Method for SynthesizeScrollGesture {
            const NAME: &'static str = "Input.synthesizeScrollGesture";
            type ReturnObject = SynthesizeScrollGestureReturnObject;
        }
        impl Method for SynthesizeTapGesture {
            const NAME: &'static str = "Input.synthesizeTapGesture";
            type ReturnObject = SynthesizeTapGestureReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DragInterceptedEvent {
                pub params: DragInterceptedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DragInterceptedEventParams {
                pub data: super::DragData,
            }
        }
    }
    pub mod Inspector {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        impl Method for Disable {
            const NAME: &'static str = "Inspector.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Inspector.enable";
            type ReturnObject = EnableReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DetachedEvent {
                pub params: DetachedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DetachedEventParams {
                #[serde(default)]
                pub reason: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct TargetCrashedEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct TargetReloadedAfterCrashEvent(pub Option<serde_json::Value>);
        }
    }
    pub mod LayerTree {
        use super::types::*;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type LayerId = String;
        pub type SnapshotId = String;
        pub type PaintProfile = Vec<JsFloat>;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum ScrollRectType {
            RepaintsOnScroll,
            TouchEventHandler,
            WheelEventHandler,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ScrollRect {
            pub rect: DOM::Rect,
            pub Type: ScrollRectType,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StickyPositionConstraint {
            pub sticky_box_rect: DOM::Rect,
            pub containing_block_rect: DOM::Rect,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub nearest_layer_shifting_sticky_box: Option<LayerId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub nearest_layer_shifting_containing_block: Option<LayerId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PictureTile {
            #[serde(default)]
            pub x: JsFloat,
            #[serde(default)]
            pub y: JsFloat,
            #[serde(default)]
            pub picture: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Layer {
            pub layer_id: LayerId,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub parent_layer_id: Option<LayerId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(default)]
            pub offset_x: JsFloat,
            #[serde(default)]
            pub offset_y: JsFloat,
            #[serde(default)]
            pub width: JsFloat,
            #[serde(default)]
            pub height: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub transform: Option<Vec<JsFloat>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub anchor_x: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub anchor_y: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub anchor_z: Option<JsFloat>,
            #[serde(default)]
            pub paint_count: JsUInt,
            #[serde(default)]
            pub draws_content: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub invisible: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub scroll_rects: Option<Vec<ScrollRect>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub sticky_position_constraint: Option<StickyPositionConstraint>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CompositingReasons {
            pub layer_id: LayerId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LoadSnapshot {
            pub tiles: Vec<PictureTile>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct MakeSnapshot {
            pub layer_id: LayerId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ProfileSnapshot {
            pub snapshot_id: SnapshotId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub min_repeat_count: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub min_duration: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub clip_rect: Option<DOM::Rect>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReleaseSnapshot {
            pub snapshot_id: SnapshotId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReplaySnapshot {
            pub snapshot_id: SnapshotId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub from_step: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub to_step: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub scale: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SnapshotCommandLog {
            pub snapshot_id: SnapshotId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CompositingReasonsReturnObject {
            pub compositing_reasons: Vec<String>,
            pub compositing_reason_ids: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LoadSnapshotReturnObject {
            pub snapshot_id: SnapshotId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct MakeSnapshotReturnObject {
            pub snapshot_id: SnapshotId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ProfileSnapshotReturnObject {
            pub timings: Vec<PaintProfile>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReleaseSnapshotReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReplaySnapshotReturnObject {
            #[serde(default)]
            pub data_url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SnapshotCommandLogReturnObject {}
        impl Method for CompositingReasons {
            const NAME: &'static str = "LayerTree.compositingReasons";
            type ReturnObject = CompositingReasonsReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "LayerTree.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "LayerTree.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for LoadSnapshot {
            const NAME: &'static str = "LayerTree.loadSnapshot";
            type ReturnObject = LoadSnapshotReturnObject;
        }
        impl Method for MakeSnapshot {
            const NAME: &'static str = "LayerTree.makeSnapshot";
            type ReturnObject = MakeSnapshotReturnObject;
        }
        impl Method for ProfileSnapshot {
            const NAME: &'static str = "LayerTree.profileSnapshot";
            type ReturnObject = ProfileSnapshotReturnObject;
        }
        impl Method for ReleaseSnapshot {
            const NAME: &'static str = "LayerTree.releaseSnapshot";
            type ReturnObject = ReleaseSnapshotReturnObject;
        }
        impl Method for ReplaySnapshot {
            const NAME: &'static str = "LayerTree.replaySnapshot";
            type ReturnObject = ReplaySnapshotReturnObject;
        }
        impl Method for SnapshotCommandLog {
            const NAME: &'static str = "LayerTree.snapshotCommandLog";
            type ReturnObject = SnapshotCommandLogReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LayerPaintedEvent {
                pub params: LayerPaintedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct LayerPaintedEventParams {
                pub layer_id: super::LayerId,
                pub clip: super::super::DOM::Rect,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LayerTreeDidChangeEvent {
                pub params: LayerTreeDidChangeEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct LayerTreeDidChangeEventParams {
                #[serde(skip_serializing_if = "Option::is_none")]
                pub layers: Option<Vec<super::Layer>>,
            }
        }
    }
    pub mod Log {
        use super::types::*;
        use super::Network;
        use super::Runtime;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum LogEntrySource {
            Xml,
            Javascript,
            Network,
            Storage,
            Appcache,
            Rendering,
            Security,
            Deprecation,
            Worker,
            Violation,
            Intervention,
            Recommendation,
            Other,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum LogEntryLevel {
            Verbose,
            Info,
            Warning,
            Error,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum LogEntryCategory {
            Cors,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ViolationSettingName {
            LongTask,
            LongLayout,
            BlockedEvent,
            BlockedParser,
            DiscouragedApiUse,
            Handler,
            RecurringHandler,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LogEntry {
            pub source: LogEntrySource,
            pub level: LogEntryLevel,
            #[serde(default)]
            pub text: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub category: Option<LogEntryCategory>,
            pub timestamp: Runtime::Timestamp,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub line_number: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stack_trace: Option<Runtime::StackTrace>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub network_request_id: Option<Network::RequestId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub worker_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub args: Option<Vec<Runtime::RemoteObject>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ViolationSetting {
            pub name: ViolationSettingName,
            #[serde(default)]
            pub threshold: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Clear(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartViolationsReport {
            pub config: Vec<ViolationSetting>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopViolationsReport(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartViolationsReportReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopViolationsReportReturnObject {}
        impl Method for Clear {
            const NAME: &'static str = "Log.clear";
            type ReturnObject = ClearReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Log.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Log.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for StartViolationsReport {
            const NAME: &'static str = "Log.startViolationsReport";
            type ReturnObject = StartViolationsReportReturnObject;
        }
        impl Method for StopViolationsReport {
            const NAME: &'static str = "Log.stopViolationsReport";
            type ReturnObject = StopViolationsReportReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct EntryAddedEvent {
                pub params: EntryAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct EntryAddedEventParams {
                pub entry: super::LogEntry,
            }
        }
    }
    pub mod Memory {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum PressureLevel {
            Moderate,
            Critical,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SamplingProfileNode {
            #[serde(default)]
            pub size: JsFloat,
            #[serde(default)]
            pub total: JsFloat,
            #[serde(default)]
            pub stack: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SamplingProfile {
            pub samples: Vec<SamplingProfileNode>,
            pub modules: Vec<Module>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Module {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub uuid: String,
            #[serde(default)]
            pub base_address: String,
            #[serde(default)]
            pub size: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetDOMCounters(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PrepareForLeakDetection(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ForciblyPurgeJavaScriptMemory(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPressureNotificationsSuppressed {
            #[serde(default)]
            pub suppressed: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SimulatePressureNotification {
            pub level: PressureLevel,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartSampling {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub sampling_interval: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub suppress_randomness: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopSampling(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAllTimeSamplingProfile(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBrowserSamplingProfile(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetSamplingProfile(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetDOMCountersReturnObject {
            #[serde(default)]
            pub documents: JsUInt,
            #[serde(default)]
            pub nodes: JsUInt,
            #[serde(default)]
            pub js_event_listeners: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PrepareForLeakDetectionReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ForciblyPurgeJavaScriptMemoryReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPressureNotificationsSuppressedReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SimulatePressureNotificationReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartSamplingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopSamplingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAllTimeSamplingProfileReturnObject {
            pub profile: SamplingProfile,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBrowserSamplingProfileReturnObject {
            pub profile: SamplingProfile,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetSamplingProfileReturnObject {
            pub profile: SamplingProfile,
        }
        impl Method for GetDOMCounters {
            const NAME: &'static str = "Memory.getDOMCounters";
            type ReturnObject = GetDOMCountersReturnObject;
        }
        impl Method for PrepareForLeakDetection {
            const NAME: &'static str = "Memory.prepareForLeakDetection";
            type ReturnObject = PrepareForLeakDetectionReturnObject;
        }
        impl Method for ForciblyPurgeJavaScriptMemory {
            const NAME: &'static str = "Memory.forciblyPurgeJavaScriptMemory";
            type ReturnObject = ForciblyPurgeJavaScriptMemoryReturnObject;
        }
        impl Method for SetPressureNotificationsSuppressed {
            const NAME: &'static str = "Memory.setPressureNotificationsSuppressed";
            type ReturnObject = SetPressureNotificationsSuppressedReturnObject;
        }
        impl Method for SimulatePressureNotification {
            const NAME: &'static str = "Memory.simulatePressureNotification";
            type ReturnObject = SimulatePressureNotificationReturnObject;
        }
        impl Method for StartSampling {
            const NAME: &'static str = "Memory.startSampling";
            type ReturnObject = StartSamplingReturnObject;
        }
        impl Method for StopSampling {
            const NAME: &'static str = "Memory.stopSampling";
            type ReturnObject = StopSamplingReturnObject;
        }
        impl Method for GetAllTimeSamplingProfile {
            const NAME: &'static str = "Memory.getAllTimeSamplingProfile";
            type ReturnObject = GetAllTimeSamplingProfileReturnObject;
        }
        impl Method for GetBrowserSamplingProfile {
            const NAME: &'static str = "Memory.getBrowserSamplingProfile";
            type ReturnObject = GetBrowserSamplingProfileReturnObject;
        }
        impl Method for GetSamplingProfile {
            const NAME: &'static str = "Memory.getSamplingProfile";
            type ReturnObject = GetSamplingProfileReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod Network {
        use super::types::*;
        use super::Debugger;
        use super::Emulation;
        use super::Network;
        use super::Page;
        use super::Runtime;
        use super::Security;
        use super::IO;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type LoaderId = String;
        pub type RequestId = String;
        pub type InterceptionId = String;
        pub type TimeSinceEpoch = JsFloat;
        pub type MonotonicTime = JsFloat;
        pub type ReportId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum ResourceType {
            Document,
            Stylesheet,
            Image,
            Media,
            Font,
            Script,
            TextTrack,
            Xhr,
            Fetch,
            EventSource,
            WebSocket,
            Manifest,
            SignedExchange,
            Ping,
            CspViolationReport,
            Preflight,
            Other,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum ErrorReason {
            Failed,
            Aborted,
            TimedOut,
            AccessDenied,
            ConnectionClosed,
            ConnectionReset,
            ConnectionRefused,
            ConnectionAborted,
            ConnectionFailed,
            NameNotResolved,
            InternetDisconnected,
            AddressUnreachable,
            BlockedByClient,
            BlockedByResponse,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ConnectionType {
            None,
            Cellular2G,
            Cellular3G,
            Cellular4G,
            Bluetooth,
            Ethernet,
            Wifi,
            Wimax,
            Other,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum CookieSameSite {
            Strict,
            Lax,
            None,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum CookiePriority {
            Low,
            Medium,
            High,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum CookieSourceScheme {
            Unset,
            NonSecure,
            Secure,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum ResourcePriority {
            VeryLow,
            Low,
            Medium,
            High,
            VeryHigh,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum RequestReferrerPolicy {
            UnsafeUrl,
            NoReferrerWhenDowngrade,
            NoReferrer,
            Origin,
            OriginWhenCrossOrigin,
            SameOrigin,
            StrictOrigin,
            StrictOriginWhenCrossOrigin,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum CertificateTransparencyCompliance {
            Unknown,
            NotCompliant,
            Compliant,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum BlockedReason {
            Other,
            Csp,
            MixedContent,
            Origin,
            Inspector,
            SubresourceFilter,
            ContentType,
            CoepFrameResourceNeedsCoepHeader,
            CoopSandboxedIframeCannotNavigateToCoopPage,
            CorpNotSameOrigin,
            CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
            CorpNotSameSite,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum CorsError {
            DisallowedByMode,
            InvalidResponse,
            WildcardOriginNotAllowed,
            MissingAllowOriginHeader,
            MultipleAllowOriginValues,
            InvalidAllowOriginValue,
            AllowOriginMismatch,
            InvalidAllowCredentials,
            CorsDisabledScheme,
            PreflightInvalidStatus,
            PreflightDisallowedRedirect,
            PreflightWildcardOriginNotAllowed,
            PreflightMissingAllowOriginHeader,
            PreflightMultipleAllowOriginValues,
            PreflightInvalidAllowOriginValue,
            PreflightAllowOriginMismatch,
            PreflightInvalidAllowCredentials,
            PreflightMissingAllowExternal,
            PreflightInvalidAllowExternal,
            InvalidAllowMethodsPreflightResponse,
            InvalidAllowHeadersPreflightResponse,
            MethodDisallowedByPreflightResponse,
            HeaderDisallowedByPreflightResponse,
            RedirectContainsCredentials,
            InsecurePrivateNetwork,
            InvalidPrivateNetworkAccess,
            UnexpectedPrivateNetworkAccess,
            NoCorsRedirectModeNotFollow,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum ServiceWorkerResponseSource {
            CacheStorage,
            HttpCache,
            FallbackCode,
            Network,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum TrustTokenParamsRefreshPolicy {
            UseCached,
            Refresh,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum TrustTokenOperationType {
            Issuance,
            Redemption,
            Signing,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum InitiatorType {
            Parser,
            Script,
            Preload,
            SignedExchange,
            Preflight,
            Other,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum SetCookieBlockedReason {
            SecureOnly,
            SameSiteStrict,
            SameSiteLax,
            SameSiteUnspecifiedTreatedAsLax,
            SameSiteNoneInsecure,
            UserPreferences,
            SyntaxError,
            SchemeNotSupported,
            OverwriteSecure,
            InvalidDomain,
            InvalidPrefix,
            UnknownError,
            SchemefulSameSiteStrict,
            SchemefulSameSiteLax,
            SchemefulSameSiteUnspecifiedTreatedAsLax,
            SamePartyFromCrossPartyContext,
            SamePartyConflictsWithOtherAttributes,
            NameValuePairExceedsMaxSize,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum CookieBlockedReason {
            SecureOnly,
            NotOnPath,
            DomainMismatch,
            SameSiteStrict,
            SameSiteLax,
            SameSiteUnspecifiedTreatedAsLax,
            SameSiteNoneInsecure,
            UserPreferences,
            UnknownError,
            SchemefulSameSiteStrict,
            SchemefulSameSiteLax,
            SchemefulSameSiteUnspecifiedTreatedAsLax,
            SamePartyFromCrossPartyContext,
            NameValuePairExceedsMaxSize,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum AuthChallengeSource {
            Server,
            Proxy,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum AuthChallengeResponseResponse {
            Default,
            CancelAuth,
            ProvideCredentials,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum InterceptionStage {
            Request,
            HeadersReceived,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum SignedExchangeErrorField {
            SignatureSig,
            SignatureIntegrity,
            SignatureCertUrl,
            SignatureCertSha256,
            SignatureValidityUrl,
            SignatureTimestamps,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ContentEncoding {
            Deflate,
            Gzip,
            Br,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum PrivateNetworkRequestPolicy {
            Allow,
            BlockFromInsecureToMorePrivate,
            WarnFromInsecureToMorePrivate,
            PreflightBlock,
            PreflightWarn,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum IPAddressSpace {
            Local,
            Private,
            Public,
            Unknown,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum CrossOriginOpenerPolicyValue {
            SameOrigin,
            SameOriginAllowPopups,
            UnsafeNone,
            SameOriginPlusCoep,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum CrossOriginEmbedderPolicyValue {
            None,
            Credentialless,
            RequireCorp,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum ReportStatus {
            Queued,
            Pending,
            MarkedForRemoval,
            Success,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum TrustTokenOperationDoneEventStatusOption {
            Ok,
            InvalidArgument,
            FailedPrecondition,
            ResourceExhausted,
            AlreadyExists,
            Unavailable,
            BadResponse,
            InternalError,
            UnknownError,
            FulfilledLocally,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Headers(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResourceTiming {
            #[serde(default)]
            pub request_time: JsFloat,
            #[serde(default)]
            pub proxy_start: JsFloat,
            #[serde(default)]
            pub proxy_end: JsFloat,
            #[serde(default)]
            pub dns_start: JsFloat,
            #[serde(default)]
            pub dns_end: JsFloat,
            #[serde(default)]
            pub connect_start: JsFloat,
            #[serde(default)]
            pub connect_end: JsFloat,
            #[serde(default)]
            pub ssl_start: JsFloat,
            #[serde(default)]
            pub ssl_end: JsFloat,
            #[serde(default)]
            pub worker_start: JsFloat,
            #[serde(default)]
            pub worker_ready: JsFloat,
            #[serde(default)]
            pub worker_fetch_start: JsFloat,
            #[serde(default)]
            pub worker_respond_with_settled: JsFloat,
            #[serde(default)]
            pub send_start: JsFloat,
            #[serde(default)]
            pub send_end: JsFloat,
            #[serde(default)]
            pub push_start: JsFloat,
            #[serde(default)]
            pub push_end: JsFloat,
            #[serde(default)]
            pub receive_headers_end: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PostDataEntry {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub bytes: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Request {
            #[serde(default)]
            pub url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url_fragment: Option<String>,
            #[serde(default)]
            pub method: String,
            pub headers: Headers,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub post_data: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub has_post_data: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub post_data_entries: Option<Vec<PostDataEntry>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mixed_content_Type: Option<Security::MixedContentType>,
            pub initial_priority: ResourcePriority,
            pub referrer_policy: RequestReferrerPolicy,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub is_link_preload: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub trust_token_params: Option<TrustTokenParams>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub is_same_site: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SignedCertificateTimestamp {
            #[serde(default)]
            pub status: String,
            #[serde(default)]
            pub origin: String,
            #[serde(default)]
            pub log_description: String,
            #[serde(default)]
            pub log_id: String,
            #[serde(default)]
            pub timestamp: JsFloat,
            #[serde(default)]
            pub hash_algorithm: String,
            #[serde(default)]
            pub signature_algorithm: String,
            #[serde(default)]
            pub signature_data: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SecurityDetails {
            #[serde(default)]
            pub protocol: String,
            #[serde(default)]
            pub key_exchange: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub key_exchange_group: Option<String>,
            #[serde(default)]
            pub cipher: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub mac: Option<String>,
            pub certificate_id: Security::CertificateId,
            #[serde(default)]
            pub subject_name: String,
            #[serde(default)]
            pub san_list: Vec<String>,
            #[serde(default)]
            pub issuer: String,
            pub valid_from: TimeSinceEpoch,
            pub valid_to: TimeSinceEpoch,
            pub signed_certificate_timestamp_list: Vec<SignedCertificateTimestamp>,
            pub certificate_transparency_compliance: CertificateTransparencyCompliance,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CorsErrorStatus {
            pub cors_error: CorsError,
            #[serde(default)]
            pub failed_parameter: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TrustTokenParams {
            pub Type: TrustTokenOperationType,
            pub refresh_policy: TrustTokenParamsRefreshPolicy,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub issuers: Option<Vec<String>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Response {
            #[serde(default)]
            pub url: String,
            #[serde(default)]
            pub status: JsUInt,
            #[serde(default)]
            pub status_text: String,
            pub headers: Headers,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub headers_text: Option<String>,
            #[serde(default)]
            pub mime_type: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub request_headers: Option<Headers>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub request_headers_text: Option<String>,
            #[serde(default)]
            pub connection_reused: bool,
            #[serde(default)]
            pub connection_id: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub remote_ip_address: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub remote_port: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub from_disk_cache: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub from_service_worker: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub from_prefetch_cache: Option<bool>,
            #[serde(default)]
            pub encoded_data_length: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub timing: Option<ResourceTiming>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub service_worker_response_source: Option<ServiceWorkerResponseSource>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub response_time: Option<TimeSinceEpoch>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub cache_storage_cache_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub protocol: Option<String>,
            pub security_state: Security::SecurityState,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub security_details: Option<SecurityDetails>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct WebSocketRequest {
            pub headers: Headers,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct WebSocketResponse {
            #[serde(default)]
            pub status: JsUInt,
            #[serde(default)]
            pub status_text: String,
            pub headers: Headers,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub headers_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub request_headers: Option<Headers>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub request_headers_text: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct WebSocketFrame {
            #[serde(default)]
            pub opcode: JsFloat,
            #[serde(default)]
            pub mask: bool,
            #[serde(default)]
            pub payload_data: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CachedResource {
            #[serde(default)]
            pub url: String,
            pub Type: ResourceType,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub response: Option<Response>,
            #[serde(default)]
            pub body_size: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Initiator {
            pub Type: InitiatorType,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stack: Option<Runtime::StackTrace>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub line_number: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub column_number: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub request_id: Option<RequestId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Cookie {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub value: String,
            #[serde(default)]
            pub domain: String,
            #[serde(default)]
            pub path: String,
            #[serde(default)]
            pub expires: JsFloat,
            #[serde(default)]
            pub size: JsUInt,
            #[serde(default)]
            pub http_only: bool,
            #[serde(default)]
            pub secure: bool,
            #[serde(default)]
            pub session: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub same_site: Option<CookieSameSite>,
            pub priority: CookiePriority,
            #[serde(default)]
            pub same_party: bool,
            pub source_scheme: CookieSourceScheme,
            #[serde(default)]
            pub source_port: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub partition_key: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub partition_key_opaque: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BlockedSetCookieWithReason {
            pub blocked_reasons: Vec<SetCookieBlockedReason>,
            #[serde(default)]
            pub cookie_line: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cookie: Option<Cookie>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BlockedCookieWithReason {
            pub blocked_reasons: Vec<CookieBlockedReason>,
            pub cookie: Cookie,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CookieParam {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub value: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub domain: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub path: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub secure: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub http_only: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub same_site: Option<CookieSameSite>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub expires: Option<TimeSinceEpoch>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub priority: Option<CookiePriority>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub same_party: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub source_scheme: Option<CookieSourceScheme>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub source_port: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub partition_key: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthChallenge {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub source: Option<AuthChallengeSource>,
            #[serde(default)]
            pub origin: String,
            #[serde(default)]
            pub scheme: String,
            #[serde(default)]
            pub realm: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthChallengeResponse {
            pub response: AuthChallengeResponseResponse,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub username: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub password: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestPattern {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url_pattern: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub resource_Type: Option<ResourceType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub interception_stage: Option<InterceptionStage>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SignedExchangeSignature {
            #[serde(default)]
            pub label: String,
            #[serde(default)]
            pub signature: String,
            #[serde(default)]
            pub integrity: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub cert_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub cert_sha_256: Option<String>,
            #[serde(default)]
            pub validity_url: String,
            #[serde(default)]
            pub date: JsUInt,
            #[serde(default)]
            pub expires: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub certificates: Option<Vec<String>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SignedExchangeHeader {
            #[serde(default)]
            pub request_url: String,
            #[serde(default)]
            pub response_code: JsUInt,
            pub response_headers: Headers,
            pub signatures: Vec<SignedExchangeSignature>,
            #[serde(default)]
            pub header_integrity: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SignedExchangeError {
            #[serde(default)]
            pub message: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub signature_index: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub error_field: Option<SignedExchangeErrorField>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SignedExchangeInfo {
            pub outer_response: Response,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub header: Option<SignedExchangeHeader>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub security_details: Option<SecurityDetails>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub errors: Option<Vec<SignedExchangeError>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ConnectTiming {
            #[serde(default)]
            pub request_time: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClientSecurityState {
            #[serde(default)]
            pub initiator_is_secure_context: bool,
            pub initiator_ip_address_space: IPAddressSpace,
            pub private_network_request_policy: PrivateNetworkRequestPolicy,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CrossOriginOpenerPolicyStatus {
            pub value: CrossOriginOpenerPolicyValue,
            pub report_only_value: CrossOriginOpenerPolicyValue,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub reporting_endpoint: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub report_only_reporting_endpoint: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CrossOriginEmbedderPolicyStatus {
            pub value: CrossOriginEmbedderPolicyValue,
            pub report_only_value: CrossOriginEmbedderPolicyValue,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub reporting_endpoint: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub report_only_reporting_endpoint: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SecurityIsolationStatus {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub coop: Option<CrossOriginOpenerPolicyStatus>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub coep: Option<CrossOriginEmbedderPolicyStatus>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReportingApiReport {
            pub id: ReportId,
            #[serde(default)]
            pub initiator_url: String,
            #[serde(default)]
            pub destination: String,
            #[serde(default)]
            pub Type: String,
            pub timestamp: Network::TimeSinceEpoch,
            #[serde(default)]
            pub depth: JsUInt,
            #[serde(default)]
            pub completed_attempts: JsUInt,
            pub status: ReportStatus,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReportingApiEndpoint {
            #[serde(default)]
            pub url: String,
            #[serde(default)]
            pub group_name: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LoadNetworkResourcePageResult {
            #[serde(default)]
            pub success: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub net_error: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub net_error_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub http_status_code: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stream: Option<IO::StreamHandle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub headers: Option<Network::Headers>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LoadNetworkResourceOptions {
            #[serde(default)]
            pub disable_cache: bool,
            #[serde(default)]
            pub include_credentials: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAcceptedEncodings {
            pub encodings: Vec<ContentEncoding>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearAcceptedEncodingsOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CanClearBrowserCache(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CanClearBrowserCookies(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CanEmulateNetworkConditions(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearBrowserCache(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearBrowserCookies(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContinueInterceptedRequest {
            pub interception_id: InterceptionId,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub error_reason: Option<ErrorReason>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub raw_response: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub method: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub post_data: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub headers: Option<Headers>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub auth_challenge_response: Option<AuthChallengeResponse>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteCookies {
            #[serde(default)]
            pub name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub domain: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub path: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EmulateNetworkConditions {
            #[serde(default)]
            pub offline: bool,
            #[serde(default)]
            pub latency: JsFloat,
            #[serde(default)]
            pub download_throughput: JsFloat,
            #[serde(default)]
            pub upload_throughput: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub connection_Type: Option<ConnectionType>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub max_total_buffer_size: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub max_resource_buffer_size: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub max_post_data_size: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAllCookies(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCertificate {
            #[serde(default)]
            pub origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCookies {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub urls: Option<Vec<String>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetResponseBody {
            pub request_id: RequestId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetRequestPostData {
            pub request_id: RequestId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetResponseBodyForInterception {
            pub interception_id: InterceptionId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeResponseBodyForInterceptionAsStream {
            pub interception_id: InterceptionId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReplayXHR {
            pub request_id: RequestId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SearchInResponseBody {
            pub request_id: RequestId,
            #[serde(default)]
            pub query: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub case_sensitive: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub is_regex: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBlockedURLs {
            #[serde(default)]
            pub urls: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBypassServiceWorker {
            #[serde(default)]
            pub bypass: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCacheDisabled {
            #[serde(default)]
            pub cache_disabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCookie {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub value: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub domain: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub path: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub secure: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub http_only: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub same_site: Option<CookieSameSite>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub expires: Option<TimeSinceEpoch>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub priority: Option<CookiePriority>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub same_party: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub source_scheme: Option<CookieSourceScheme>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub source_port: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub partition_key: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCookies {
            pub cookies: Vec<CookieParam>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetExtraHTTPHeaders {
            pub headers: Headers,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAttachDebugStack {
            #[serde(default)]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetRequestInterception {
            pub patterns: Vec<RequestPattern>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetUserAgentOverride {
            #[serde(default)]
            pub user_agent: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub accept_language: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub platform: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub user_agent_metadata: Option<Emulation::UserAgentMetadata>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetSecurityIsolationStatus {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub frame_id: Option<Page::FrameId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReportingApi {
            #[serde(default)]
            pub enable: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LoadNetworkResource {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub frame_id: Option<Page::FrameId>,
            #[serde(default)]
            pub url: String,
            pub options: LoadNetworkResourceOptions,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAcceptedEncodingsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearAcceptedEncodingsOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CanClearBrowserCacheReturnObject {
            #[serde(default)]
            pub result: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CanClearBrowserCookiesReturnObject {
            #[serde(default)]
            pub result: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CanEmulateNetworkConditionsReturnObject {
            #[serde(default)]
            pub result: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearBrowserCacheReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearBrowserCookiesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContinueInterceptedRequestReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteCookiesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EmulateNetworkConditionsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAllCookiesReturnObject {
            pub cookies: Vec<Cookie>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCertificateReturnObject {
            pub table_names: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCookiesReturnObject {
            pub cookies: Vec<Cookie>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetResponseBodyReturnObject {
            #[serde(default)]
            pub body: String,
            #[serde(default)]
            pub base_64_encoded: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetRequestPostDataReturnObject {
            #[serde(default)]
            pub post_data: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetResponseBodyForInterceptionReturnObject {
            #[serde(default)]
            pub body: String,
            #[serde(default)]
            pub base_64_encoded: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeResponseBodyForInterceptionAsStreamReturnObject {
            pub stream: IO::StreamHandle,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReplayXHRReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SearchInResponseBodyReturnObject {
            pub result: Debugger::SearchMatch,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBlockedURLsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBypassServiceWorkerReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCacheDisabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCookieReturnObject {
            #[serde(default)]
            pub success: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCookiesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetExtraHTTPHeadersReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAttachDebugStackReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetRequestInterceptionReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetUserAgentOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetSecurityIsolationStatusReturnObject {
            pub status: SecurityIsolationStatus,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReportingApiReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LoadNetworkResourceReturnObject {
            pub resource: LoadNetworkResourcePageResult,
        }
        impl Method for SetAcceptedEncodings {
            const NAME: &'static str = "Network.setAcceptedEncodings";
            type ReturnObject = SetAcceptedEncodingsReturnObject;
        }
        impl Method for ClearAcceptedEncodingsOverride {
            const NAME: &'static str = "Network.clearAcceptedEncodingsOverride";
            type ReturnObject = ClearAcceptedEncodingsOverrideReturnObject;
        }
        impl Method for CanClearBrowserCache {
            const NAME: &'static str = "Network.canClearBrowserCache";
            type ReturnObject = CanClearBrowserCacheReturnObject;
        }
        impl Method for CanClearBrowserCookies {
            const NAME: &'static str = "Network.canClearBrowserCookies";
            type ReturnObject = CanClearBrowserCookiesReturnObject;
        }
        impl Method for CanEmulateNetworkConditions {
            const NAME: &'static str = "Network.canEmulateNetworkConditions";
            type ReturnObject = CanEmulateNetworkConditionsReturnObject;
        }
        impl Method for ClearBrowserCache {
            const NAME: &'static str = "Network.clearBrowserCache";
            type ReturnObject = ClearBrowserCacheReturnObject;
        }
        impl Method for ClearBrowserCookies {
            const NAME: &'static str = "Network.clearBrowserCookies";
            type ReturnObject = ClearBrowserCookiesReturnObject;
        }
        impl Method for ContinueInterceptedRequest {
            const NAME: &'static str = "Network.continueInterceptedRequest";
            type ReturnObject = ContinueInterceptedRequestReturnObject;
        }
        impl Method for DeleteCookies {
            const NAME: &'static str = "Network.deleteCookies";
            type ReturnObject = DeleteCookiesReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Network.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for EmulateNetworkConditions {
            const NAME: &'static str = "Network.emulateNetworkConditions";
            type ReturnObject = EmulateNetworkConditionsReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Network.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetAllCookies {
            const NAME: &'static str = "Network.getAllCookies";
            type ReturnObject = GetAllCookiesReturnObject;
        }
        impl Method for GetCertificate {
            const NAME: &'static str = "Network.getCertificate";
            type ReturnObject = GetCertificateReturnObject;
        }
        impl Method for GetCookies {
            const NAME: &'static str = "Network.getCookies";
            type ReturnObject = GetCookiesReturnObject;
        }
        impl Method for GetResponseBody {
            const NAME: &'static str = "Network.getResponseBody";
            type ReturnObject = GetResponseBodyReturnObject;
        }
        impl Method for GetRequestPostData {
            const NAME: &'static str = "Network.getRequestPostData";
            type ReturnObject = GetRequestPostDataReturnObject;
        }
        impl Method for GetResponseBodyForInterception {
            const NAME: &'static str = "Network.getResponseBodyForInterception";
            type ReturnObject = GetResponseBodyForInterceptionReturnObject;
        }
        impl Method for TakeResponseBodyForInterceptionAsStream {
            const NAME: &'static str = "Network.takeResponseBodyForInterceptionAsStream";
            type ReturnObject = TakeResponseBodyForInterceptionAsStreamReturnObject;
        }
        impl Method for ReplayXHR {
            const NAME: &'static str = "Network.replayXHR";
            type ReturnObject = ReplayXHRReturnObject;
        }
        impl Method for SearchInResponseBody {
            const NAME: &'static str = "Network.searchInResponseBody";
            type ReturnObject = SearchInResponseBodyReturnObject;
        }
        impl Method for SetBlockedURLs {
            const NAME: &'static str = "Network.setBlockedURLs";
            type ReturnObject = SetBlockedURLsReturnObject;
        }
        impl Method for SetBypassServiceWorker {
            const NAME: &'static str = "Network.setBypassServiceWorker";
            type ReturnObject = SetBypassServiceWorkerReturnObject;
        }
        impl Method for SetCacheDisabled {
            const NAME: &'static str = "Network.setCacheDisabled";
            type ReturnObject = SetCacheDisabledReturnObject;
        }
        impl Method for SetCookie {
            const NAME: &'static str = "Network.setCookie";
            type ReturnObject = SetCookieReturnObject;
        }
        impl Method for SetCookies {
            const NAME: &'static str = "Network.setCookies";
            type ReturnObject = SetCookiesReturnObject;
        }
        impl Method for SetExtraHTTPHeaders {
            const NAME: &'static str = "Network.setExtraHTTPHeaders";
            type ReturnObject = SetExtraHTTPHeadersReturnObject;
        }
        impl Method for SetAttachDebugStack {
            const NAME: &'static str = "Network.setAttachDebugStack";
            type ReturnObject = SetAttachDebugStackReturnObject;
        }
        impl Method for SetRequestInterception {
            const NAME: &'static str = "Network.setRequestInterception";
            type ReturnObject = SetRequestInterceptionReturnObject;
        }
        impl Method for SetUserAgentOverride {
            const NAME: &'static str = "Network.setUserAgentOverride";
            type ReturnObject = SetUserAgentOverrideReturnObject;
        }
        impl Method for GetSecurityIsolationStatus {
            const NAME: &'static str = "Network.getSecurityIsolationStatus";
            type ReturnObject = GetSecurityIsolationStatusReturnObject;
        }
        impl Method for EnableReportingApi {
            const NAME: &'static str = "Network.enableReportingApi";
            type ReturnObject = EnableReportingApiReturnObject;
        }
        impl Method for LoadNetworkResource {
            const NAME: &'static str = "Network.loadNetworkResource";
            type ReturnObject = LoadNetworkResourceReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DataReceivedEvent {
                pub params: DataReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DataReceivedEventParams {
                pub request_id: super::RequestId,
                pub timestamp: super::MonotonicTime,
                #[serde(default)]
                pub data_length: JsUInt,
                #[serde(default)]
                pub encoded_data_length: JsUInt,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct EventSourceMessageReceivedEvent {
                pub params: EventSourceMessageReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct EventSourceMessageReceivedEventParams {
                pub request_id: super::RequestId,
                pub timestamp: super::MonotonicTime,
                #[serde(default)]
                pub event_name: String,
                #[serde(default)]
                pub event_id: String,
                #[serde(default)]
                pub data: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LoadingFailedEvent {
                pub params: LoadingFailedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct LoadingFailedEventParams {
                pub request_id: super::RequestId,
                pub timestamp: super::MonotonicTime,
                pub Type: super::ResourceType,
                #[serde(default)]
                pub error_text: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub canceled: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub blocked_reason: Option<super::BlockedReason>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub cors_error_status: Option<super::CorsErrorStatus>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LoadingFinishedEvent {
                pub params: LoadingFinishedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct LoadingFinishedEventParams {
                pub request_id: super::RequestId,
                pub timestamp: super::MonotonicTime,
                #[serde(default)]
                pub encoded_data_length: JsFloat,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub should_report_corb_blocking: Option<bool>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RequestInterceptedEvent {
                pub params: RequestInterceptedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct RequestInterceptedEventParams {
                pub interception_id: super::InterceptionId,
                pub request: super::Request,
                pub frame_id: super::super::Page::FrameId,
                pub resource_Type: super::ResourceType,
                #[serde(default)]
                pub is_navigation_request: bool,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub is_download: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub redirect_url: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub auth_challenge: Option<super::AuthChallenge>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub response_error_reason: Option<super::ErrorReason>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub response_status_code: Option<JsUInt>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub response_headers: Option<super::Headers>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub request_id: Option<super::RequestId>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RequestServedFromCacheEvent {
                pub params: RequestServedFromCacheEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct RequestServedFromCacheEventParams {
                pub request_id: super::RequestId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RequestWillBeSentEvent {
                pub params: RequestWillBeSentEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct RequestWillBeSentEventParams {
                pub request_id: super::RequestId,
                pub loader_id: super::LoaderId,
                #[serde(default)]
                pub document_url: String,
                pub request: super::Request,
                pub timestamp: super::MonotonicTime,
                pub wall_time: super::TimeSinceEpoch,
                pub initiator: super::Initiator,
                #[serde(default)]
                pub redirect_has_extra_info: bool,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub redirect_response: Option<super::Response>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub Type: Option<super::ResourceType>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub frame_id: Option<super::super::Page::FrameId>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub has_user_gesture: Option<bool>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ResourceChangedPriorityEvent {
                pub params: ResourceChangedPriorityEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ResourceChangedPriorityEventParams {
                pub request_id: super::RequestId,
                pub new_priority: super::ResourcePriority,
                pub timestamp: super::MonotonicTime,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SignedExchangeReceivedEvent {
                pub params: SignedExchangeReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct SignedExchangeReceivedEventParams {
                pub request_id: super::RequestId,
                pub info: super::SignedExchangeInfo,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ResponseReceivedEvent {
                pub params: ResponseReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ResponseReceivedEventParams {
                pub request_id: super::RequestId,
                pub loader_id: super::LoaderId,
                pub timestamp: super::MonotonicTime,
                pub Type: super::ResourceType,
                pub response: super::Response,
                #[serde(default)]
                pub has_extra_info: bool,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub frame_id: Option<super::super::Page::FrameId>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketClosedEvent {
                pub params: WebSocketClosedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct WebSocketClosedEventParams {
                pub request_id: super::RequestId,
                pub timestamp: super::MonotonicTime,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketCreatedEvent {
                pub params: WebSocketCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct WebSocketCreatedEventParams {
                pub request_id: super::RequestId,
                #[serde(default)]
                pub url: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub initiator: Option<super::Initiator>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketFrameErrorEvent {
                pub params: WebSocketFrameErrorEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct WebSocketFrameErrorEventParams {
                pub request_id: super::RequestId,
                pub timestamp: super::MonotonicTime,
                #[serde(default)]
                pub error_message: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketFrameReceivedEvent {
                pub params: WebSocketFrameReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct WebSocketFrameReceivedEventParams {
                pub request_id: super::RequestId,
                pub timestamp: super::MonotonicTime,
                pub response: super::WebSocketFrame,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketFrameSentEvent {
                pub params: WebSocketFrameSentEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct WebSocketFrameSentEventParams {
                pub request_id: super::RequestId,
                pub timestamp: super::MonotonicTime,
                pub response: super::WebSocketFrame,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketHandshakeResponseReceivedEvent {
                pub params: WebSocketHandshakeResponseReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct WebSocketHandshakeResponseReceivedEventParams {
                pub request_id: super::RequestId,
                pub timestamp: super::MonotonicTime,
                pub response: super::WebSocketResponse,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebSocketWillSendHandshakeRequestEvent {
                pub params: WebSocketWillSendHandshakeRequestEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct WebSocketWillSendHandshakeRequestEventParams {
                pub request_id: super::RequestId,
                pub timestamp: super::MonotonicTime,
                pub wall_time: super::TimeSinceEpoch,
                pub request: super::WebSocketRequest,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebTransportCreatedEvent {
                pub params: WebTransportCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct WebTransportCreatedEventParams {
                pub transport_id: super::RequestId,
                #[serde(default)]
                pub url: String,
                pub timestamp: super::MonotonicTime,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub initiator: Option<super::Initiator>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebTransportConnectionEstablishedEvent {
                pub params: WebTransportConnectionEstablishedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct WebTransportConnectionEstablishedEventParams {
                pub transport_id: super::RequestId,
                pub timestamp: super::MonotonicTime,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WebTransportClosedEvent {
                pub params: WebTransportClosedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct WebTransportClosedEventParams {
                pub transport_id: super::RequestId,
                pub timestamp: super::MonotonicTime,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RequestWillBeSentExtraInfoEvent {
                pub params: RequestWillBeSentExtraInfoEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct RequestWillBeSentExtraInfoEventParams {
                pub request_id: super::RequestId,
                pub associated_cookies: Vec<super::BlockedCookieWithReason>,
                pub headers: super::Headers,
                pub connect_timing: super::ConnectTiming,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub client_security_state: Option<super::ClientSecurityState>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ResponseReceivedExtraInfoEvent {
                pub params: ResponseReceivedExtraInfoEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ResponseReceivedExtraInfoEventParams {
                pub request_id: super::RequestId,
                pub blocked_cookies: Vec<super::BlockedSetCookieWithReason>,
                pub headers: super::Headers,
                pub resource_ip_address_space: super::IPAddressSpace,
                #[serde(default)]
                pub status_code: JsUInt,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub headers_text: Option<String>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TrustTokenOperationDoneEvent {
                pub params: TrustTokenOperationDoneEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct TrustTokenOperationDoneEventParams {
                pub status: super::TrustTokenOperationDoneEventStatusOption,
                pub Type: super::TrustTokenOperationType,
                pub request_id: super::RequestId,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub top_level_origin: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub issuer_origin: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub issued_token_count: Option<JsUInt>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SubresourceWebBundleMetadataReceivedEvent {
                pub params: SubresourceWebBundleMetadataReceivedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct SubresourceWebBundleMetadataReceivedEventParams {
                pub request_id: super::RequestId,
                #[serde(default)]
                pub urls: Vec<String>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SubresourceWebBundleMetadataErrorEvent {
                pub params: SubresourceWebBundleMetadataErrorEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct SubresourceWebBundleMetadataErrorEventParams {
                pub request_id: super::RequestId,
                #[serde(default)]
                pub error_message: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SubresourceWebBundleInnerResponseParsedEvent {
                pub params: SubresourceWebBundleInnerResponseParsedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct SubresourceWebBundleInnerResponseParsedEventParams {
                pub inner_request_id: super::RequestId,
                #[serde(default)]
                pub inner_request_url: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub bundle_request_id: Option<super::RequestId>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SubresourceWebBundleInnerResponseErrorEvent {
                pub params: SubresourceWebBundleInnerResponseErrorEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct SubresourceWebBundleInnerResponseErrorEventParams {
                pub inner_request_id: super::RequestId,
                #[serde(default)]
                pub inner_request_url: String,
                #[serde(default)]
                pub error_message: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub bundle_request_id: Option<super::RequestId>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ReportingApiReportAddedEvent {
                pub params: ReportingApiReportAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ReportingApiReportAddedEventParams {
                pub report: super::ReportingApiReport,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ReportingApiReportUpdatedEvent {
                pub params: ReportingApiReportUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ReportingApiReportUpdatedEventParams {
                pub report: super::ReportingApiReport,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ReportingApiEndpointsChangedForOriginEvent {
                pub params: ReportingApiEndpointsChangedForOriginEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ReportingApiEndpointsChangedForOriginEventParams {
                #[serde(default)]
                pub origin: String,
                pub endpoints: Vec<super::ReportingApiEndpoint>,
            }
        }
    }
    pub mod Overlay {
        use super::types::*;
        use super::Page;
        use super::Runtime;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum LineStylePattern {
            Dashed,
            Dotted,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ContrastAlgorithm {
            Aa,
            Aaa,
            Apca,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ColorFormat {
            Rgb,
            Hsl,
            Hex,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum InspectMode {
            SearchForNode,
            SearchForUaShadowDom,
            CaptureAreaScreenshot,
            ShowDistances,
            None,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SourceOrderConfig {
            pub parent_outline_color: DOM::RGBA,
            pub child_outline_color: DOM::RGBA,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GridHighlightConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub show_grid_extension_lines: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub show_positive_line_numbers: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub show_negative_line_numbers: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub show_area_names: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub show_line_names: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub show_track_sizes: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub grid_border_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cell_border_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub row_line_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub column_line_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub grid_border_dash: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub cell_border_dash: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub row_line_dash: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub column_line_dash: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub row_gap_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub row_hatch_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub column_gap_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub column_hatch_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub area_border_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub grid_background_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FlexContainerHighlightConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub container_border: Option<LineStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub line_separator: Option<LineStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub item_separator: Option<LineStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub main_distributed_space: Option<BoxStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cross_distributed_space: Option<BoxStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub row_gap_space: Option<BoxStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub column_gap_space: Option<BoxStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cross_alignment: Option<LineStyle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FlexItemHighlightConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub base_size_box: Option<BoxStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub base_size_border: Option<LineStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub flexibility_arrow: Option<LineStyle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LineStyle {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub pattern: Option<LineStylePattern>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BoxStyle {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fill_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub hatch_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub show_info: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub show_styles: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub show_rulers: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub show_accessibility_info: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub show_extension_lines: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub content_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub padding_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub border_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub margin_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub event_target_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub shape_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub shape_margin_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub css_grid_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub color_format: Option<ColorFormat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub grid_highlight_config: Option<GridHighlightConfig>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub flex_container_highlight_config: Option<FlexContainerHighlightConfig>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub flex_item_highlight_config: Option<FlexItemHighlightConfig>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub contrast_algorithm: Option<ContrastAlgorithm>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub container_query_container_highlight_config:
                Option<ContainerQueryContainerHighlightConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GridNodeHighlightConfig {
            pub grid_highlight_config: GridHighlightConfig,
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FlexNodeHighlightConfig {
            pub flex_container_highlight_config: FlexContainerHighlightConfig,
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ScrollSnapContainerHighlightConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub snapport_border: Option<LineStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub snap_area_border: Option<LineStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub scroll_margin_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub scroll_padding_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ScrollSnapHighlightConfig {
            pub scroll_snap_container_highlight_config: ScrollSnapContainerHighlightConfig,
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HingeConfig {
            pub rect: DOM::Rect,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub content_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub outline_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContainerQueryHighlightConfig {
            pub container_query_container_highlight_config: ContainerQueryContainerHighlightConfig,
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContainerQueryContainerHighlightConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub container_border: Option<LineStyle>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub descendant_border: Option<LineStyle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct IsolatedElementHighlightConfig {
            pub isolation_mode_highlight_config: IsolationModeHighlightConfig,
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct IsolationModeHighlightConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub resizer_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub resizer_handle_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mask_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetHighlightObjectForTest {
            pub node_id: DOM::NodeId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub include_distance: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub include_style: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub color_format: Option<ColorFormat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub show_accessibility_info: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetGridHighlightObjectsForTest {
            pub node_ids: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetSourceOrderHighlightObjectForTest {
            pub node_id: DOM::NodeId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HideHighlight(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightFrame {
            pub frame_id: Page::FrameId,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub content_color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub content_outline_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightNode {
            pub highlight_config: HighlightConfig,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<DOM::NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub object_id: Option<Runtime::RemoteObjectId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub selector: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightQuad {
            pub quad: DOM::Quad,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub outline_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightRect {
            #[serde(default)]
            pub x: JsUInt,
            #[serde(default)]
            pub y: JsUInt,
            #[serde(default)]
            pub width: JsUInt,
            #[serde(default)]
            pub height: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub color: Option<DOM::RGBA>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub outline_color: Option<DOM::RGBA>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightSourceOrder {
            pub source_order_config: SourceOrderConfig,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<DOM::NodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub backend_node_id: Option<DOM::BackendNodeId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub object_id: Option<Runtime::RemoteObjectId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInspectMode {
            pub mode: InspectMode,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub highlight_config: Option<HighlightConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowAdHighlights {
            #[serde(default)]
            pub show: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPausedInDebuggerMessage {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub message: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowDebugBorders {
            #[serde(default)]
            pub show: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowFPSCounter {
            #[serde(default)]
            pub show: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowGridOverlays {
            pub grid_node_highlight_configs: Vec<GridNodeHighlightConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowFlexOverlays {
            pub flex_node_highlight_configs: Vec<FlexNodeHighlightConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowScrollSnapOverlays {
            pub scroll_snap_highlight_configs: Vec<ScrollSnapHighlightConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowContainerQueryOverlays {
            pub container_query_highlight_configs: Vec<ContainerQueryHighlightConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowPaintRects {
            #[serde(default)]
            pub result: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowLayoutShiftRegions {
            #[serde(default)]
            pub result: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowScrollBottleneckRects {
            #[serde(default)]
            pub show: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowHitTestBorders {
            #[serde(default)]
            pub show: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowWebVitals {
            #[serde(default)]
            pub show: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowViewportSizeOnResize {
            #[serde(default)]
            pub show: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowHinge {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub hinge_config: Option<HingeConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowIsolatedElements {
            pub isolated_element_highlight_configs: Vec<IsolatedElementHighlightConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetHighlightObjectForTestReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetGridHighlightObjectsForTestReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetSourceOrderHighlightObjectForTestReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HideHighlightReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightFrameReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightNodeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightQuadReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightRectReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HighlightSourceOrderReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInspectModeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowAdHighlightsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetPausedInDebuggerMessageReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowDebugBordersReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowFPSCounterReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowGridOverlaysReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowFlexOverlaysReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowScrollSnapOverlaysReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowContainerQueryOverlaysReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowPaintRectsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowLayoutShiftRegionsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowScrollBottleneckRectsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowHitTestBordersReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowWebVitalsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowViewportSizeOnResizeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowHingeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetShowIsolatedElementsReturnObject {}
        impl Method for Disable {
            const NAME: &'static str = "Overlay.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Overlay.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetHighlightObjectForTest {
            const NAME: &'static str = "Overlay.getHighlightObjectForTest";
            type ReturnObject = GetHighlightObjectForTestReturnObject;
        }
        impl Method for GetGridHighlightObjectsForTest {
            const NAME: &'static str = "Overlay.getGridHighlightObjectsForTest";
            type ReturnObject = GetGridHighlightObjectsForTestReturnObject;
        }
        impl Method for GetSourceOrderHighlightObjectForTest {
            const NAME: &'static str = "Overlay.getSourceOrderHighlightObjectForTest";
            type ReturnObject = GetSourceOrderHighlightObjectForTestReturnObject;
        }
        impl Method for HideHighlight {
            const NAME: &'static str = "Overlay.hideHighlight";
            type ReturnObject = HideHighlightReturnObject;
        }
        impl Method for HighlightFrame {
            const NAME: &'static str = "Overlay.highlightFrame";
            type ReturnObject = HighlightFrameReturnObject;
        }
        impl Method for HighlightNode {
            const NAME: &'static str = "Overlay.highlightNode";
            type ReturnObject = HighlightNodeReturnObject;
        }
        impl Method for HighlightQuad {
            const NAME: &'static str = "Overlay.highlightQuad";
            type ReturnObject = HighlightQuadReturnObject;
        }
        impl Method for HighlightRect {
            const NAME: &'static str = "Overlay.highlightRect";
            type ReturnObject = HighlightRectReturnObject;
        }
        impl Method for HighlightSourceOrder {
            const NAME: &'static str = "Overlay.highlightSourceOrder";
            type ReturnObject = HighlightSourceOrderReturnObject;
        }
        impl Method for SetInspectMode {
            const NAME: &'static str = "Overlay.setInspectMode";
            type ReturnObject = SetInspectModeReturnObject;
        }
        impl Method for SetShowAdHighlights {
            const NAME: &'static str = "Overlay.setShowAdHighlights";
            type ReturnObject = SetShowAdHighlightsReturnObject;
        }
        impl Method for SetPausedInDebuggerMessage {
            const NAME: &'static str = "Overlay.setPausedInDebuggerMessage";
            type ReturnObject = SetPausedInDebuggerMessageReturnObject;
        }
        impl Method for SetShowDebugBorders {
            const NAME: &'static str = "Overlay.setShowDebugBorders";
            type ReturnObject = SetShowDebugBordersReturnObject;
        }
        impl Method for SetShowFPSCounter {
            const NAME: &'static str = "Overlay.setShowFPSCounter";
            type ReturnObject = SetShowFPSCounterReturnObject;
        }
        impl Method for SetShowGridOverlays {
            const NAME: &'static str = "Overlay.setShowGridOverlays";
            type ReturnObject = SetShowGridOverlaysReturnObject;
        }
        impl Method for SetShowFlexOverlays {
            const NAME: &'static str = "Overlay.setShowFlexOverlays";
            type ReturnObject = SetShowFlexOverlaysReturnObject;
        }
        impl Method for SetShowScrollSnapOverlays {
            const NAME: &'static str = "Overlay.setShowScrollSnapOverlays";
            type ReturnObject = SetShowScrollSnapOverlaysReturnObject;
        }
        impl Method for SetShowContainerQueryOverlays {
            const NAME: &'static str = "Overlay.setShowContainerQueryOverlays";
            type ReturnObject = SetShowContainerQueryOverlaysReturnObject;
        }
        impl Method for SetShowPaintRects {
            const NAME: &'static str = "Overlay.setShowPaintRects";
            type ReturnObject = SetShowPaintRectsReturnObject;
        }
        impl Method for SetShowLayoutShiftRegions {
            const NAME: &'static str = "Overlay.setShowLayoutShiftRegions";
            type ReturnObject = SetShowLayoutShiftRegionsReturnObject;
        }
        impl Method for SetShowScrollBottleneckRects {
            const NAME: &'static str = "Overlay.setShowScrollBottleneckRects";
            type ReturnObject = SetShowScrollBottleneckRectsReturnObject;
        }
        impl Method for SetShowHitTestBorders {
            const NAME: &'static str = "Overlay.setShowHitTestBorders";
            type ReturnObject = SetShowHitTestBordersReturnObject;
        }
        impl Method for SetShowWebVitals {
            const NAME: &'static str = "Overlay.setShowWebVitals";
            type ReturnObject = SetShowWebVitalsReturnObject;
        }
        impl Method for SetShowViewportSizeOnResize {
            const NAME: &'static str = "Overlay.setShowViewportSizeOnResize";
            type ReturnObject = SetShowViewportSizeOnResizeReturnObject;
        }
        impl Method for SetShowHinge {
            const NAME: &'static str = "Overlay.setShowHinge";
            type ReturnObject = SetShowHingeReturnObject;
        }
        impl Method for SetShowIsolatedElements {
            const NAME: &'static str = "Overlay.setShowIsolatedElements";
            type ReturnObject = SetShowIsolatedElementsReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct InspectNodeRequestedEvent {
                pub params: InspectNodeRequestedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct InspectNodeRequestedEventParams {
                pub backend_node_id: super::super::DOM::BackendNodeId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodeHighlightRequestedEvent {
                pub params: NodeHighlightRequestedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct NodeHighlightRequestedEventParams {
                pub node_id: super::super::DOM::NodeId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ScreenshotRequestedEvent {
                pub params: ScreenshotRequestedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ScreenshotRequestedEventParams {
                pub viewport: super::super::Page::Viewport,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct InspectModeCanceledEvent(pub Option<serde_json::Value>);
        }
    }
    pub mod Page {
        use super::types::*;
        use super::Debugger;
        use super::Emulation;
        use super::Network;
        use super::Runtime;
        use super::DOM;
        use super::IO;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type FrameId = String;
        pub type ScriptIdentifier = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum AdFrameType {
            None,
            Child,
            Root,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum AdFrameExplanation {
            ParentIsAd,
            CreatedByAdScript,
            MatchedBlockingRule,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum SecureContextType {
            Secure,
            SecureLocalhost,
            InsecureScheme,
            InsecureAncestor,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum CrossOriginIsolatedContextType {
            Isolated,
            NotIsolated,
            NotIsolatedFeatureDisabled,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum GatedAPIFeatures {
            SharedArrayBuffers,
            SharedArrayBuffersTransferAllowed,
            PerformanceMeasureMemory,
            PerformanceProfile,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum PermissionsPolicyFeature {
            Accelerometer,
            AmbientLightSensor,
            AttributionReporting,
            Autoplay,
            Camera,
            ChDpr,
            ChDeviceMemory,
            ChDownlink,
            ChEct,
            ChPrefersColorScheme,
            ChRtt,
            ChUa,
            ChUaArch,
            ChUaBitness,
            ChUaPlatform,
            ChUaModel,
            ChUaMobile,
            ChUaFullVersion,
            ChUaFullVersionList,
            ChUaPlatformVersion,
            ChUaReduced,
            ChViewportHeight,
            ChViewportWidth,
            ChWidth,
            ClipboardRead,
            ClipboardWrite,
            CrossOriginIsolated,
            DirectSockets,
            DisplayCapture,
            DocumentDomain,
            EncryptedMedia,
            ExecutionWhileOutOfViewport,
            ExecutionWhileNotRendered,
            FocusWithoutUserActivation,
            Fullscreen,
            Frobulate,
            Gamepad,
            Geolocation,
            Gyroscope,
            Hid,
            IdleDetection,
            InterestCohort,
            JoinAdInterestGroup,
            KeyboardMap,
            Magnetometer,
            Microphone,
            Midi,
            OtpCredentials,
            Payment,
            PictureInPicture,
            PublickeyCredentialsGet,
            RunAdAuction,
            ScreenWakeLock,
            Serial,
            SharedAutofill,
            StorageAccessApi,
            SyncXhr,
            TrustTokenRedemption,
            Usb,
            VerticalScroll,
            WebShare,
            WindowPlacement,
            XrSpatialTracking,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum PermissionsPolicyBlockReason {
            Header,
            IframeAttribute,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum OriginTrialTokenStatus {
            Success,
            NotSupported,
            Insecure,
            Expired,
            WrongOrigin,
            InvalidSignature,
            Malformed,
            WrongVersion,
            FeatureDisabled,
            TokenDisabled,
            FeatureDisabledForUser,
            UnknownTrial,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum OriginTrialStatus {
            Enabled,
            ValidTokenNotProvided,
            OsNotSupported,
            TrialNotAllowed,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum OriginTrialUsageRestriction {
            None,
            Subset,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum TransitionType {
            Link,
            Typed,
            AddressBar,
            AutoBookmark,
            AutoSubframe,
            ManualSubframe,
            Generated,
            AutoToplevel,
            FormSubmit,
            Reload,
            Keyword,
            KeywordGenerated,
            Other,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum DialogType {
            Alert,
            Confirm,
            Prompt,
            Beforeunload,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ClientNavigationReason {
            FormSubmissionGet,
            FormSubmissionPost,
            HttpHeaderRefresh,
            ScriptInitiated,
            MetaTagRefresh,
            PageBlockInterstitial,
            Reload,
            AnchorClick,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ClientNavigationDisposition {
            CurrentTab,
            NewTab,
            NewWindow,
            Download,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ReferrerPolicy {
            NoReferrer,
            NoReferrerWhenDowngrade,
            Origin,
            OriginWhenCrossOrigin,
            SameOrigin,
            StrictOrigin,
            StrictOriginWhenCrossOrigin,
            UnsafeUrl,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum NavigationType {
            Navigation,
            BackForwardCacheRestore,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum BackForwardCacheNotRestoredReason {
            NotMainFrame,
            BackForwardCacheDisabled,
            RelatedActiveContentsExist,
            HttpStatusNotOk,
            SchemeNotHttpOrHttps,
            Loading,
            WasGrantedMediaAccess,
            DisableForRenderFrameHostCalled,
            DomainNotAllowed,
            HttpMethodNotGet,
            SubframeIsNavigating,
            Timeout,
            CacheLimit,
            JavaScriptExecution,
            RendererProcessKilled,
            RendererProcessCrashed,
            GrantedMediaStreamAccess,
            SchedulerTrackedFeatureUsed,
            ConflictingBrowsingInstance,
            CacheFlushed,
            ServiceWorkerVersionActivation,
            SessionRestored,
            ServiceWorkerPostMessage,
            EnteredBackForwardCacheBeforeServiceWorkerHostAdded,
            RenderFrameHostReusedSameSite,
            RenderFrameHostReusedCrossSite,
            ServiceWorkerClaim,
            IgnoreEventAndEvict,
            HaveInnerContents,
            TimeoutPuttingInCache,
            BackForwardCacheDisabledByLowMemory,
            BackForwardCacheDisabledByCommandLine,
            NetworkRequestDatapipeDrainedAsBytesConsumer,
            NetworkRequestRedirected,
            NetworkRequestTimeout,
            NetworkExceedsBufferLimit,
            NavigationCancelledWhileRestoring,
            NotMostRecentNavigationEntry,
            BackForwardCacheDisabledForPrerender,
            UserAgentOverrideDiffers,
            ForegroundCacheLimit,
            BrowsingInstanceNotSwapped,
            BackForwardCacheDisabledForDelegate,
            OptInUnloadHeaderNotPresent,
            UnloadHandlerExistsInMainFrame,
            UnloadHandlerExistsInSubFrame,
            ServiceWorkerUnregistration,
            CacheControlNoStore,
            CacheControlNoStoreCookieModified,
            CacheControlNoStoreHttpOnlyCookieModified,
            NoResponseHead,
            Unknown,
            ActivationNavigationsDisallowedForBug1234857,
            WebSocket,
            WebTransport,
            WebRtc,
            MainResourceHasCacheControlNoStore,
            MainResourceHasCacheControlNoCache,
            SubresourceHasCacheControlNoStore,
            SubresourceHasCacheControlNoCache,
            ContainsPlugins,
            DocumentLoaded,
            DedicatedWorkerOrWorklet,
            OutstandingNetworkRequestOthers,
            OutstandingIndexedDbTransaction,
            RequestedNotificationsPermission,
            RequestedMidiPermission,
            RequestedAudioCapturePermission,
            RequestedVideoCapturePermission,
            RequestedBackForwardCacheBlockedSensors,
            RequestedBackgroundWorkPermission,
            BroadcastChannel,
            IndexedDbConnection,
            WebXr,
            SharedWorker,
            WebLocks,
            WebHid,
            WebShare,
            RequestedStorageAccessGrant,
            WebNfc,
            OutstandingNetworkRequestFetch,
            OutstandingNetworkRequestXhr,
            AppBanner,
            Printing,
            WebDatabase,
            PictureInPicture,
            Portal,
            SpeechRecognizer,
            IdleManager,
            PaymentManager,
            SpeechSynthesis,
            KeyboardLock,
            WebOtpService,
            OutstandingNetworkRequestDirectSocket,
            InjectedJavascript,
            InjectedStyleSheet,
            Dummy,
            ContentSecurityHandler,
            ContentWebAuthenticationApi,
            ContentFileChooser,
            ContentSerial,
            ContentFileSystemAccess,
            ContentMediaDevicesDispatcherHost,
            ContentWebBluetooth,
            ContentWebUsb,
            ContentMediaSession,
            ContentMediaSessionService,
            EmbedderPopupBlockerTabHelper,
            EmbedderSafeBrowsingTriggeredPopupBlocker,
            EmbedderSafeBrowsingThreatDetails,
            EmbedderAppBannerManager,
            EmbedderDomDistillerViewerSource,
            EmbedderDomDistillerSelfDeletingRequestDelegate,
            EmbedderOomInterventionTabHelper,
            EmbedderOfflinePage,
            EmbedderChromePasswordManagerClientBindCredentialManager,
            EmbedderPermissionRequestManager,
            EmbedderModalDialog,
            EmbedderExtensions,
            EmbedderExtensionMessaging,
            EmbedderExtensionMessagingForOpenPort,
            EmbedderExtensionSentMessageToCachedFrame,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum BackForwardCacheNotRestoredReasonType {
            SupportPending,
            PageSupportNeeded,
            Circumstantial,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum CaptureScreenshotFormatOption {
            Jpeg,
            Png,
            Webp,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum CaptureSnapshotFormatOption {
            Mhtml,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum PrintToPDFTransfer_modeOption {
            ReturnAsBase64,
            ReturnAsStream,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum SetDownloadBehaviorBehaviorOption {
            Deny,
            Allow,
            Default,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum SetTouchEmulationEnabledConfigurationOption {
            Mobile,
            Desktop,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum StartScreencastFormatOption {
            Jpeg,
            Png,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum SetWebLifecycleStateStateOption {
            Frozen,
            Active,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum SetSPCTransactionModeModeOption {
            None,
            Autoaccept,
            Autoreject,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum FileChooserOpenedEventModeOption {
            SelectSingle,
            SelectMultiple,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum FrameDetachedEventReasonOption {
            Remove,
            Swap,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum DownloadProgressEventStateOption {
            InProgress,
            Completed,
            Canceled,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AdFrameStatus {
            pub ad_frame_Type: AdFrameType,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub explanations: Option<Vec<AdFrameExplanation>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PermissionsPolicyBlockLocator {
            pub frame_id: FrameId,
            pub block_reason: PermissionsPolicyBlockReason,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PermissionsPolicyFeatureState {
            pub feature: PermissionsPolicyFeature,
            #[serde(default)]
            pub allowed: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub locator: Option<PermissionsPolicyBlockLocator>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct OriginTrialToken {
            #[serde(default)]
            pub origin: String,
            #[serde(default)]
            pub match_sub_domains: bool,
            #[serde(default)]
            pub trial_name: String,
            pub expiry_time: Network::TimeSinceEpoch,
            #[serde(default)]
            pub is_third_party: bool,
            pub usage_restriction: OriginTrialUsageRestriction,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct OriginTrialTokenWithStatus {
            #[serde(default)]
            pub raw_token_text: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub parsed_token: Option<OriginTrialToken>,
            pub status: OriginTrialTokenStatus,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct OriginTrial {
            #[serde(default)]
            pub trial_name: String,
            pub status: OriginTrialStatus,
            pub tokens_with_status: Vec<OriginTrialTokenWithStatus>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Frame {
            pub id: FrameId,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub parent_id: Option<FrameId>,
            pub loader_id: Network::LoaderId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub name: Option<String>,
            #[serde(default)]
            pub url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url_fragment: Option<String>,
            #[serde(default)]
            pub domain_and_registry: String,
            #[serde(default)]
            pub security_origin: String,
            #[serde(default)]
            pub mime_type: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub unreachable_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ad_frame_status: Option<AdFrameStatus>,
            pub secure_context_Type: SecureContextType,
            pub cross_origin_isolated_context_Type: CrossOriginIsolatedContextType,
            pub gated_api_features: Vec<GatedAPIFeatures>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FrameResource {
            #[serde(default)]
            pub url: String,
            pub Type: Network::ResourceType,
            #[serde(default)]
            pub mime_type: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub last_modified: Option<Network::TimeSinceEpoch>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub content_size: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub failed: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub canceled: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FrameResourceTree {
            pub frame: Frame,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub child_frames: Option<Vec<FrameResourceTree>>,
            pub resources: Vec<FrameResource>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FrameTree {
            pub frame: Frame,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub child_frames: Option<Vec<FrameTree>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct NavigationEntry {
            #[serde(default)]
            pub id: JsUInt,
            #[serde(default)]
            pub url: String,
            #[serde(default)]
            pub user_typed_url: String,
            #[serde(default)]
            pub title: String,
            pub transition_Type: TransitionType,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ScreencastFrameMetadata {
            #[serde(default)]
            pub offset_top: JsFloat,
            #[serde(default)]
            pub page_scale_factor: JsFloat,
            #[serde(default)]
            pub device_width: JsFloat,
            #[serde(default)]
            pub device_height: JsFloat,
            #[serde(default)]
            pub scroll_offset_x: JsFloat,
            #[serde(default)]
            pub scroll_offset_y: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub timestamp: Option<Network::TimeSinceEpoch>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AppManifestError {
            #[serde(default)]
            pub message: String,
            #[serde(default)]
            pub critical: JsUInt,
            #[serde(default)]
            pub line: JsUInt,
            #[serde(default)]
            pub column: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AppManifestParsedProperties {
            #[serde(default)]
            pub scope: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LayoutViewport {
            #[serde(default)]
            pub page_x: JsUInt,
            #[serde(default)]
            pub page_y: JsUInt,
            #[serde(default)]
            pub client_width: JsUInt,
            #[serde(default)]
            pub client_height: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct VisualViewport {
            #[serde(default)]
            pub offset_x: JsFloat,
            #[serde(default)]
            pub offset_y: JsFloat,
            #[serde(default)]
            pub page_x: JsFloat,
            #[serde(default)]
            pub page_y: JsFloat,
            #[serde(default)]
            pub client_width: JsFloat,
            #[serde(default)]
            pub client_height: JsFloat,
            #[serde(default)]
            pub scale: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub zoom: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Viewport {
            #[serde(default)]
            pub x: JsFloat,
            #[serde(default)]
            pub y: JsFloat,
            #[serde(default)]
            pub width: JsFloat,
            #[serde(default)]
            pub height: JsFloat,
            #[serde(default)]
            pub scale: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FontFamilies {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub standard: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub fixed: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub serif: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub sans_serif: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub cursive: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub fantasy: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub pictograph: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FontSizes {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub standard: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub fixed: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct InstallabilityErrorArgument {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct InstallabilityError {
            #[serde(default)]
            pub error_id: String,
            pub error_arguments: Vec<InstallabilityErrorArgument>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CompilationCacheParams {
            #[serde(default)]
            pub url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub eager: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BackForwardCacheNotRestoredExplanation {
            pub Type: BackForwardCacheNotRestoredReasonType,
            pub reason: BackForwardCacheNotRestoredReason,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddScriptToEvaluateOnLoad {
            #[serde(default)]
            pub script_source: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddScriptToEvaluateOnNewDocument {
            #[serde(default)]
            pub source: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub world_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub include_command_line_api: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BringToFront(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CaptureScreenshot {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub format: Option<CaptureScreenshotFormatOption>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub quality: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub clip: Option<Viewport>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub from_surface: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub capture_beyond_viewport: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CaptureSnapshot {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub format: Option<CaptureSnapshotFormatOption>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceMetricsOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceOrientationOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearGeolocationOverride(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CreateIsolatedWorld {
            pub frame_id: FrameId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub world_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub grant_univeral_access: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteCookie {
            #[serde(default)]
            pub cookie_name: String,
            #[serde(default)]
            pub url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAppManifest(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetInstallabilityErrors(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetManifestIcons(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAppId(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCookies(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetFrameTree(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetLayoutMetrics(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetNavigationHistory(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResetNavigationHistory(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetResourceContent {
            pub frame_id: FrameId,
            #[serde(default)]
            pub url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetResourceTree(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HandleJavaScriptDialog {
            #[serde(default)]
            pub accept: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub prompt_text: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Navigate {
            #[serde(default)]
            pub url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub referrer: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub transition_Type: Option<TransitionType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub frame_id: Option<FrameId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub referrer_policy: Option<ReferrerPolicy>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct NavigateToHistoryEntry {
            #[serde(default)]
            pub entry_id: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PrintToPDF {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub landscape: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub display_header_footer: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub print_background: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub scale: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub paper_width: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub paper_height: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub margin_top: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub margin_bottom: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub margin_left: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub margin_right: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub page_ranges: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub ignore_invalid_page_ranges: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub header_template: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub footer_template: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub prefer_css_page_size: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub transfer_mode: Option<PrintToPDFTransfer_modeOption>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Reload {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub ignore_cache: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub script_to_evaluate_on_load: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveScriptToEvaluateOnLoad {
            pub identifier: ScriptIdentifier,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveScriptToEvaluateOnNewDocument {
            pub identifier: ScriptIdentifier,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ScreencastFrameAck {
            #[serde(default)]
            pub session_id: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SearchInResource {
            pub frame_id: FrameId,
            #[serde(default)]
            pub url: String,
            #[serde(default)]
            pub query: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub case_sensitive: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub is_regex: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAdBlockingEnabled {
            #[serde(default)]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBypassCSP {
            #[serde(default)]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetPermissionsPolicyState {
            pub frame_id: FrameId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetOriginTrials {
            pub frame_id: FrameId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDeviceMetricsOverride {
            #[serde(default)]
            pub width: JsUInt,
            #[serde(default)]
            pub height: JsUInt,
            #[serde(default)]
            pub device_scale_factor: JsFloat,
            #[serde(default)]
            pub mobile: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub scale: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub screen_width: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub screen_height: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub position_x: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub position_y: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub dont_set_visible_size: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub screen_orientation: Option<Emulation::ScreenOrientation>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub viewport: Option<Viewport>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDeviceOrientationOverride {
            #[serde(default)]
            pub alpha: JsFloat,
            #[serde(default)]
            pub beta: JsFloat,
            #[serde(default)]
            pub gamma: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetFontFamilies {
            pub font_families: FontFamilies,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetFontSizes {
            pub font_sizes: FontSizes,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDocumentContent {
            pub frame_id: FrameId,
            #[serde(default)]
            pub html: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDownloadBehavior {
            pub behavior: SetDownloadBehaviorBehaviorOption,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub download_path: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetGeolocationOverride {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub latitude: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub longitude: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub accuracy: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetLifecycleEventsEnabled {
            #[serde(default)]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetTouchEmulationEnabled {
            #[serde(default)]
            pub enabled: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub configuration: Option<SetTouchEmulationEnabledConfigurationOption>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartScreencast {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub format: Option<StartScreencastFormatOption>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub quality: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub max_width: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub max_height: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub every_nth_frame: Option<JsUInt>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopLoading(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Crash(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Close(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetWebLifecycleState {
            pub state: SetWebLifecycleStateStateOption,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopScreencast(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ProduceCompilationCache {
            pub scripts: Vec<CompilationCacheParams>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddCompilationCache {
            #[serde(default)]
            pub url: String,
            #[serde(default)]
            pub data: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearCompilationCache(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetSPCTransactionMode {
            pub mode: SetSPCTransactionModeModeOption,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GenerateTestReport {
            #[serde(default)]
            pub message: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub group: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct WaitForDebugger(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInterceptFileChooserDialog {
            #[serde(default)]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddScriptToEvaluateOnLoadReturnObject {
            pub identifier: ScriptIdentifier,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddScriptToEvaluateOnNewDocumentReturnObject {
            pub identifier: ScriptIdentifier,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BringToFrontReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CaptureScreenshotReturnObject {
            #[serde(default)]
            pub data: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CaptureSnapshotReturnObject {
            #[serde(default)]
            pub data: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceMetricsOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDeviceOrientationOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearGeolocationOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CreateIsolatedWorldReturnObject {
            pub execution_context_id: Runtime::ExecutionContextId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeleteCookieReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAppManifestReturnObject {
            #[serde(default)]
            pub url: String,
            pub errors: Vec<AppManifestError>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub data: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub parsed: Option<AppManifestParsedProperties>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetInstallabilityErrorsReturnObject {
            pub installability_errors: Vec<InstallabilityError>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetManifestIconsReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub primary_icon: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetAppIdReturnObject {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub app_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub recommended_id: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCookiesReturnObject {
            pub cookies: Network::Cookie,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetFrameTreeReturnObject {
            pub frame_tree: FrameTree,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetLayoutMetricsReturnObject {
            pub layout_viewport: LayoutViewport,
            pub visual_viewport: VisualViewport,
            pub content_size: DOM::Rect,
            pub css_layout_viewport: LayoutViewport,
            pub css_visual_viewport: VisualViewport,
            pub css_content_size: DOM::Rect,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetNavigationHistoryReturnObject {
            #[serde(default)]
            pub current_index: JsUInt,
            pub entries: Vec<NavigationEntry>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ResetNavigationHistoryReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetResourceContentReturnObject {
            #[serde(default)]
            pub content: String,
            #[serde(default)]
            pub base_64_encoded: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetResourceTreeReturnObject {
            pub frame_tree: FrameResourceTree,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HandleJavaScriptDialogReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct NavigateReturnObject {
            pub frame_id: FrameId,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub loader_id: Option<Network::LoaderId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub error_text: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct NavigateToHistoryEntryReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PrintToPDFReturnObject {
            #[serde(default)]
            pub data: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stream: Option<IO::StreamHandle>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ReloadReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveScriptToEvaluateOnLoadReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveScriptToEvaluateOnNewDocumentReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ScreencastFrameAckReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SearchInResourceReturnObject {
            pub result: Debugger::SearchMatch,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAdBlockingEnabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetBypassCSPReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetPermissionsPolicyStateReturnObject {
            pub states: Vec<PermissionsPolicyFeatureState>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetOriginTrialsReturnObject {
            pub origin_trials: Vec<OriginTrial>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDeviceMetricsOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDeviceOrientationOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetFontFamiliesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetFontSizesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDocumentContentReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDownloadBehaviorReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetGeolocationOverrideReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetLifecycleEventsEnabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetTouchEmulationEnabledReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartScreencastReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopLoadingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CrashReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CloseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetWebLifecycleStateReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopScreencastReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ProduceCompilationCacheReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddCompilationCacheReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearCompilationCacheReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetSPCTransactionModeReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GenerateTestReportReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct WaitForDebuggerReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetInterceptFileChooserDialogReturnObject {}
        impl Method for AddScriptToEvaluateOnLoad {
            const NAME: &'static str = "Page.addScriptToEvaluateOnLoad";
            type ReturnObject = AddScriptToEvaluateOnLoadReturnObject;
        }
        impl Method for AddScriptToEvaluateOnNewDocument {
            const NAME: &'static str = "Page.addScriptToEvaluateOnNewDocument";
            type ReturnObject = AddScriptToEvaluateOnNewDocumentReturnObject;
        }
        impl Method for BringToFront {
            const NAME: &'static str = "Page.bringToFront";
            type ReturnObject = BringToFrontReturnObject;
        }
        impl Method for CaptureScreenshot {
            const NAME: &'static str = "Page.captureScreenshot";
            type ReturnObject = CaptureScreenshotReturnObject;
        }
        impl Method for CaptureSnapshot {
            const NAME: &'static str = "Page.captureSnapshot";
            type ReturnObject = CaptureSnapshotReturnObject;
        }
        impl Method for ClearDeviceMetricsOverride {
            const NAME: &'static str = "Page.clearDeviceMetricsOverride";
            type ReturnObject = ClearDeviceMetricsOverrideReturnObject;
        }
        impl Method for ClearDeviceOrientationOverride {
            const NAME: &'static str = "Page.clearDeviceOrientationOverride";
            type ReturnObject = ClearDeviceOrientationOverrideReturnObject;
        }
        impl Method for ClearGeolocationOverride {
            const NAME: &'static str = "Page.clearGeolocationOverride";
            type ReturnObject = ClearGeolocationOverrideReturnObject;
        }
        impl Method for CreateIsolatedWorld {
            const NAME: &'static str = "Page.createIsolatedWorld";
            type ReturnObject = CreateIsolatedWorldReturnObject;
        }
        impl Method for DeleteCookie {
            const NAME: &'static str = "Page.deleteCookie";
            type ReturnObject = DeleteCookieReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Page.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Page.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for GetAppManifest {
            const NAME: &'static str = "Page.getAppManifest";
            type ReturnObject = GetAppManifestReturnObject;
        }
        impl Method for GetInstallabilityErrors {
            const NAME: &'static str = "Page.getInstallabilityErrors";
            type ReturnObject = GetInstallabilityErrorsReturnObject;
        }
        impl Method for GetManifestIcons {
            const NAME: &'static str = "Page.getManifestIcons";
            type ReturnObject = GetManifestIconsReturnObject;
        }
        impl Method for GetAppId {
            const NAME: &'static str = "Page.getAppId";
            type ReturnObject = GetAppIdReturnObject;
        }
        impl Method for GetCookies {
            const NAME: &'static str = "Page.getCookies";
            type ReturnObject = GetCookiesReturnObject;
        }
        impl Method for GetFrameTree {
            const NAME: &'static str = "Page.getFrameTree";
            type ReturnObject = GetFrameTreeReturnObject;
        }
        impl Method for GetLayoutMetrics {
            const NAME: &'static str = "Page.getLayoutMetrics";
            type ReturnObject = GetLayoutMetricsReturnObject;
        }
        impl Method for GetNavigationHistory {
            const NAME: &'static str = "Page.getNavigationHistory";
            type ReturnObject = GetNavigationHistoryReturnObject;
        }
        impl Method for ResetNavigationHistory {
            const NAME: &'static str = "Page.resetNavigationHistory";
            type ReturnObject = ResetNavigationHistoryReturnObject;
        }
        impl Method for GetResourceContent {
            const NAME: &'static str = "Page.getResourceContent";
            type ReturnObject = GetResourceContentReturnObject;
        }
        impl Method for GetResourceTree {
            const NAME: &'static str = "Page.getResourceTree";
            type ReturnObject = GetResourceTreeReturnObject;
        }
        impl Method for HandleJavaScriptDialog {
            const NAME: &'static str = "Page.handleJavaScriptDialog";
            type ReturnObject = HandleJavaScriptDialogReturnObject;
        }
        impl Method for Navigate {
            const NAME: &'static str = "Page.navigate";
            type ReturnObject = NavigateReturnObject;
        }
        impl Method for NavigateToHistoryEntry {
            const NAME: &'static str = "Page.navigateToHistoryEntry";
            type ReturnObject = NavigateToHistoryEntryReturnObject;
        }
        impl Method for PrintToPDF {
            const NAME: &'static str = "Page.printToPDF";
            type ReturnObject = PrintToPDFReturnObject;
        }
        impl Method for Reload {
            const NAME: &'static str = "Page.reload";
            type ReturnObject = ReloadReturnObject;
        }
        impl Method for RemoveScriptToEvaluateOnLoad {
            const NAME: &'static str = "Page.removeScriptToEvaluateOnLoad";
            type ReturnObject = RemoveScriptToEvaluateOnLoadReturnObject;
        }
        impl Method for RemoveScriptToEvaluateOnNewDocument {
            const NAME: &'static str = "Page.removeScriptToEvaluateOnNewDocument";
            type ReturnObject = RemoveScriptToEvaluateOnNewDocumentReturnObject;
        }
        impl Method for ScreencastFrameAck {
            const NAME: &'static str = "Page.screencastFrameAck";
            type ReturnObject = ScreencastFrameAckReturnObject;
        }
        impl Method for SearchInResource {
            const NAME: &'static str = "Page.searchInResource";
            type ReturnObject = SearchInResourceReturnObject;
        }
        impl Method for SetAdBlockingEnabled {
            const NAME: &'static str = "Page.setAdBlockingEnabled";
            type ReturnObject = SetAdBlockingEnabledReturnObject;
        }
        impl Method for SetBypassCSP {
            const NAME: &'static str = "Page.setBypassCSP";
            type ReturnObject = SetBypassCSPReturnObject;
        }
        impl Method for GetPermissionsPolicyState {
            const NAME: &'static str = "Page.getPermissionsPolicyState";
            type ReturnObject = GetPermissionsPolicyStateReturnObject;
        }
        impl Method for GetOriginTrials {
            const NAME: &'static str = "Page.getOriginTrials";
            type ReturnObject = GetOriginTrialsReturnObject;
        }
        impl Method for SetDeviceMetricsOverride {
            const NAME: &'static str = "Page.setDeviceMetricsOverride";
            type ReturnObject = SetDeviceMetricsOverrideReturnObject;
        }
        impl Method for SetDeviceOrientationOverride {
            const NAME: &'static str = "Page.setDeviceOrientationOverride";
            type ReturnObject = SetDeviceOrientationOverrideReturnObject;
        }
        impl Method for SetFontFamilies {
            const NAME: &'static str = "Page.setFontFamilies";
            type ReturnObject = SetFontFamiliesReturnObject;
        }
        impl Method for SetFontSizes {
            const NAME: &'static str = "Page.setFontSizes";
            type ReturnObject = SetFontSizesReturnObject;
        }
        impl Method for SetDocumentContent {
            const NAME: &'static str = "Page.setDocumentContent";
            type ReturnObject = SetDocumentContentReturnObject;
        }
        impl Method for SetDownloadBehavior {
            const NAME: &'static str = "Page.setDownloadBehavior";
            type ReturnObject = SetDownloadBehaviorReturnObject;
        }
        impl Method for SetGeolocationOverride {
            const NAME: &'static str = "Page.setGeolocationOverride";
            type ReturnObject = SetGeolocationOverrideReturnObject;
        }
        impl Method for SetLifecycleEventsEnabled {
            const NAME: &'static str = "Page.setLifecycleEventsEnabled";
            type ReturnObject = SetLifecycleEventsEnabledReturnObject;
        }
        impl Method for SetTouchEmulationEnabled {
            const NAME: &'static str = "Page.setTouchEmulationEnabled";
            type ReturnObject = SetTouchEmulationEnabledReturnObject;
        }
        impl Method for StartScreencast {
            const NAME: &'static str = "Page.startScreencast";
            type ReturnObject = StartScreencastReturnObject;
        }
        impl Method for StopLoading {
            const NAME: &'static str = "Page.stopLoading";
            type ReturnObject = StopLoadingReturnObject;
        }
        impl Method for Crash {
            const NAME: &'static str = "Page.crash";
            type ReturnObject = CrashReturnObject;
        }
        impl Method for Close {
            const NAME: &'static str = "Page.close";
            type ReturnObject = CloseReturnObject;
        }
        impl Method for SetWebLifecycleState {
            const NAME: &'static str = "Page.setWebLifecycleState";
            type ReturnObject = SetWebLifecycleStateReturnObject;
        }
        impl Method for StopScreencast {
            const NAME: &'static str = "Page.stopScreencast";
            type ReturnObject = StopScreencastReturnObject;
        }
        impl Method for ProduceCompilationCache {
            const NAME: &'static str = "Page.produceCompilationCache";
            type ReturnObject = ProduceCompilationCacheReturnObject;
        }
        impl Method for AddCompilationCache {
            const NAME: &'static str = "Page.addCompilationCache";
            type ReturnObject = AddCompilationCacheReturnObject;
        }
        impl Method for ClearCompilationCache {
            const NAME: &'static str = "Page.clearCompilationCache";
            type ReturnObject = ClearCompilationCacheReturnObject;
        }
        impl Method for SetSPCTransactionMode {
            const NAME: &'static str = "Page.setSPCTransactionMode";
            type ReturnObject = SetSPCTransactionModeReturnObject;
        }
        impl Method for GenerateTestReport {
            const NAME: &'static str = "Page.generateTestReport";
            type ReturnObject = GenerateTestReportReturnObject;
        }
        impl Method for WaitForDebugger {
            const NAME: &'static str = "Page.waitForDebugger";
            type ReturnObject = WaitForDebuggerReturnObject;
        }
        impl Method for SetInterceptFileChooserDialog {
            const NAME: &'static str = "Page.setInterceptFileChooserDialog";
            type ReturnObject = SetInterceptFileChooserDialogReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DomContentEventFiredEvent {
                pub params: DomContentEventFiredEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DomContentEventFiredEventParams {
                pub timestamp: super::super::Network::MonotonicTime,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FileChooserOpenedEvent {
                pub params: FileChooserOpenedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct FileChooserOpenedEventParams {
                pub frame_id: super::FrameId,
                pub backend_node_id: super::super::DOM::BackendNodeId,
                pub mode: super::FileChooserOpenedEventModeOption,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameAttachedEvent {
                pub params: FrameAttachedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct FrameAttachedEventParams {
                pub frame_id: super::FrameId,
                pub parent_frame_id: super::FrameId,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub stack: Option<super::super::Runtime::StackTrace>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameClearedScheduledNavigationEvent {
                pub params: FrameClearedScheduledNavigationEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct FrameClearedScheduledNavigationEventParams {
                pub frame_id: super::FrameId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameDetachedEvent {
                pub params: FrameDetachedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct FrameDetachedEventParams {
                pub frame_id: super::FrameId,
                pub reason: super::FrameDetachedEventReasonOption,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameNavigatedEvent {
                pub params: FrameNavigatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct FrameNavigatedEventParams {
                pub frame: super::Frame,
                pub Type: super::NavigationType,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DocumentOpenedEvent {
                pub params: DocumentOpenedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DocumentOpenedEventParams {
                pub frame: super::Frame,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct FrameResizedEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameRequestedNavigationEvent {
                pub params: FrameRequestedNavigationEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct FrameRequestedNavigationEventParams {
                pub frame_id: super::FrameId,
                pub reason: super::ClientNavigationReason,
                #[serde(default)]
                pub url: String,
                pub disposition: super::ClientNavigationDisposition,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameScheduledNavigationEvent {
                pub params: FrameScheduledNavigationEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct FrameScheduledNavigationEventParams {
                pub frame_id: super::FrameId,
                #[serde(default)]
                pub delay: JsFloat,
                pub reason: super::ClientNavigationReason,
                #[serde(default)]
                pub url: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameStartedLoadingEvent {
                pub params: FrameStartedLoadingEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct FrameStartedLoadingEventParams {
                pub frame_id: super::FrameId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct FrameStoppedLoadingEvent {
                pub params: FrameStoppedLoadingEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct FrameStoppedLoadingEventParams {
                pub frame_id: super::FrameId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DownloadWillBeginEvent {
                pub params: DownloadWillBeginEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DownloadWillBeginEventParams {
                pub frame_id: super::FrameId,
                #[serde(default)]
                pub guid: String,
                #[serde(default)]
                pub url: String,
                #[serde(default)]
                pub suggested_filename: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DownloadProgressEvent {
                pub params: DownloadProgressEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DownloadProgressEventParams {
                #[serde(default)]
                pub guid: String,
                #[serde(default)]
                pub total_bytes: JsFloat,
                #[serde(default)]
                pub received_bytes: JsFloat,
                pub state: super::DownloadProgressEventStateOption,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct InterstitialHiddenEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct InterstitialShownEvent(pub Option<serde_json::Value>);
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct JavascriptDialogClosedEvent {
                pub params: JavascriptDialogClosedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct JavascriptDialogClosedEventParams {
                #[serde(default)]
                pub result: bool,
                #[serde(default)]
                pub user_input: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct JavascriptDialogOpeningEvent {
                pub params: JavascriptDialogOpeningEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct JavascriptDialogOpeningEventParams {
                #[serde(default)]
                pub url: String,
                #[serde(default)]
                pub message: String,
                pub Type: super::DialogType,
                #[serde(default)]
                pub has_browser_handler: bool,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub default_prompt: Option<String>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LifecycleEventEvent {
                pub params: LifecycleEventEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct LifecycleEventEventParams {
                pub frame_id: super::FrameId,
                pub loader_id: super::super::Network::LoaderId,
                #[serde(default)]
                pub name: String,
                pub timestamp: super::super::Network::MonotonicTime,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct BackForwardCacheNotUsedEvent {
                pub params: BackForwardCacheNotUsedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct BackForwardCacheNotUsedEventParams {
                pub loader_id: super::super::Network::LoaderId,
                pub frame_id: super::FrameId,
                pub not_restored_explanations: Vec<super::BackForwardCacheNotRestoredExplanation>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct LoadEventFiredEvent {
                pub params: LoadEventFiredEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct LoadEventFiredEventParams {
                pub timestamp: super::super::Network::MonotonicTime,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NavigatedWithinDocumentEvent {
                pub params: NavigatedWithinDocumentEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct NavigatedWithinDocumentEventParams {
                pub frame_id: super::FrameId,
                #[serde(default)]
                pub url: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ScreencastFrameEvent {
                pub params: ScreencastFrameEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ScreencastFrameEventParams {
                #[serde(default)]
                pub data: String,
                pub metadata: super::ScreencastFrameMetadata,
                #[serde(default)]
                pub session_id: JsUInt,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ScreencastVisibilityChangedEvent {
                pub params: ScreencastVisibilityChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ScreencastVisibilityChangedEventParams {
                #[serde(default)]
                pub visible: bool,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WindowOpenEvent {
                pub params: WindowOpenEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct WindowOpenEventParams {
                #[serde(default)]
                pub url: String,
                #[serde(default)]
                pub window_name: String,
                #[serde(default)]
                pub window_features: Vec<String>,
                #[serde(default)]
                pub user_gesture: bool,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct CompilationCacheProducedEvent {
                pub params: CompilationCacheProducedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct CompilationCacheProducedEventParams {
                #[serde(default)]
                pub url: String,
                #[serde(default)]
                pub data: String,
            }
        }
    }
    pub mod Performance {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum EnableTime_domainOption {
            TimeTicks,
            ThreadTicks,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum SetTimeDomainTime_domainOption {
            TimeTicks,
            ThreadTicks,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Metric {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub value: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub time_domain: Option<EnableTime_domainOption>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetTimeDomain {
            pub time_domain: SetTimeDomainTime_domainOption,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetMetrics(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetTimeDomainReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetMetricsReturnObject {
            pub metrics: Vec<Metric>,
        }
        impl Method for Disable {
            const NAME: &'static str = "Performance.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Performance.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for SetTimeDomain {
            const NAME: &'static str = "Performance.setTimeDomain";
            type ReturnObject = SetTimeDomainReturnObject;
        }
        impl Method for GetMetrics {
            const NAME: &'static str = "Performance.getMetrics";
            type ReturnObject = GetMetricsReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct MetricsEvent {
                pub params: MetricsEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct MetricsEventParams {
                pub metrics: Vec<super::Metric>,
                #[serde(default)]
                pub title: String,
            }
        }
    }
    pub mod PerformanceTimeline {
        use super::types::*;
        use super::Network;
        use super::Page;
        use super::DOM;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LargestContentfulPaint {
            pub render_time: Network::TimeSinceEpoch,
            pub load_time: Network::TimeSinceEpoch,
            #[serde(default)]
            pub size: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub element_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<DOM::BackendNodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LayoutShiftAttribution {
            pub previous_rect: DOM::Rect,
            pub current_rect: DOM::Rect,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub node_id: Option<DOM::BackendNodeId>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct LayoutShift {
            #[serde(default)]
            pub value: JsFloat,
            #[serde(default)]
            pub had_recent_input: bool,
            pub last_input_time: Network::TimeSinceEpoch,
            pub sources: Vec<LayoutShiftAttribution>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TimelineEvent {
            pub frame_id: Page::FrameId,
            #[serde(default)]
            pub Type: String,
            #[serde(default)]
            pub name: String,
            pub time: Network::TimeSinceEpoch,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub duration: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub lcp_details: Option<LargestContentfulPaint>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub layout_shift_details: Option<LayoutShift>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable {
            #[serde(default)]
            pub event_Types: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        impl Method for Enable {
            const NAME: &'static str = "PerformanceTimeline.enable";
            type ReturnObject = EnableReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TimelineEventAddedEvent {
                pub params: TimelineEventAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct TimelineEventAddedEventParams {
                pub event: super::TimelineEvent,
            }
        }
    }
    pub mod Security {
        use super::types::*;
        use super::Network;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type CertificateId = JsUInt;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum MixedContentType {
            Blockable,
            OptionallyBlockable,
            None,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum SecurityState {
            Unknown,
            Neutral,
            Insecure,
            Secure,
            Info,
            InsecureBroken,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum SafetyTipStatus {
            BadReputation,
            Lookalike,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum CertificateErrorAction {
            Continue,
            Cancel,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CertificateSecurityState {
            #[serde(default)]
            pub protocol: String,
            #[serde(default)]
            pub key_exchange: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub key_exchange_group: Option<String>,
            #[serde(default)]
            pub cipher: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub mac: Option<String>,
            #[serde(default)]
            pub certificate: Vec<String>,
            #[serde(default)]
            pub subject_name: String,
            #[serde(default)]
            pub issuer: String,
            pub valid_from: Network::TimeSinceEpoch,
            pub valid_to: Network::TimeSinceEpoch,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub certificate_network_error: Option<String>,
            #[serde(default)]
            pub certificate_has_weak_signature: bool,
            #[serde(default)]
            pub certificate_has_sha_1_signature: bool,
            #[serde(default)]
            pub modern_ssl: bool,
            #[serde(default)]
            pub obsolete_ssl_protocol: bool,
            #[serde(default)]
            pub obsolete_ssl_key_exchange: bool,
            #[serde(default)]
            pub obsolete_ssl_cipher: bool,
            #[serde(default)]
            pub obsolete_ssl_signature: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SafetyTipInfo {
            pub safety_tip_status: SafetyTipStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub safe_url: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct VisibleSecurityState {
            pub security_state: SecurityState,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub certificate_security_state: Option<CertificateSecurityState>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub safety_tip_info: Option<SafetyTipInfo>,
            #[serde(default)]
            pub security_state_issue_ids: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SecurityStateExplanation {
            pub security_state: SecurityState,
            #[serde(default)]
            pub title: String,
            #[serde(default)]
            pub summary: String,
            #[serde(default)]
            pub description: String,
            pub mixed_content_Type: MixedContentType,
            #[serde(default)]
            pub certificate: Vec<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub recommendations: Option<Vec<String>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct InsecureContentStatus {
            #[serde(default)]
            pub ran_mixed_content: bool,
            #[serde(default)]
            pub displayed_mixed_content: bool,
            #[serde(default)]
            pub contained_mixed_form: bool,
            #[serde(default)]
            pub ran_content_with_cert_errors: bool,
            #[serde(default)]
            pub displayed_content_with_cert_errors: bool,
            pub ran_insecure_content_style: SecurityState,
            pub displayed_insecure_content_style: SecurityState,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetIgnoreCertificateErrors {
            #[serde(default)]
            pub ignore: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HandleCertificateError {
            #[serde(default)]
            pub event_id: JsUInt,
            pub action: CertificateErrorAction,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetOverrideCertificateErrors {
            #[serde(default)]
            pub Override: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetIgnoreCertificateErrorsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HandleCertificateErrorReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetOverrideCertificateErrorsReturnObject {}
        impl Method for Disable {
            const NAME: &'static str = "Security.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Security.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for SetIgnoreCertificateErrors {
            const NAME: &'static str = "Security.setIgnoreCertificateErrors";
            type ReturnObject = SetIgnoreCertificateErrorsReturnObject;
        }
        impl Method for HandleCertificateError {
            const NAME: &'static str = "Security.handleCertificateError";
            type ReturnObject = HandleCertificateErrorReturnObject;
        }
        impl Method for SetOverrideCertificateErrors {
            const NAME: &'static str = "Security.setOverrideCertificateErrors";
            type ReturnObject = SetOverrideCertificateErrorsReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct CertificateErrorEvent {
                pub params: CertificateErrorEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct CertificateErrorEventParams {
                #[serde(default)]
                pub event_id: JsUInt,
                #[serde(default)]
                pub error_Type: String,
                #[serde(default)]
                pub request_url: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct VisibleSecurityStateChangedEvent {
                pub params: VisibleSecurityStateChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct VisibleSecurityStateChangedEventParams {
                pub visible_security_state: super::VisibleSecurityState,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct SecurityStateChangedEvent {
                pub params: SecurityStateChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct SecurityStateChangedEventParams {
                pub security_state: super::SecurityState,
                #[serde(default)]
                pub scheme_is_cryptographic: bool,
                pub explanations: Vec<super::SecurityStateExplanation>,
                pub insecure_content_status: super::InsecureContentStatus,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub summary: Option<String>,
            }
        }
    }
    pub mod ServiceWorker {
        use super::types::*;
        use super::Target;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type RegistrationID = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ServiceWorkerVersionRunningStatus {
            Stopped,
            Starting,
            Running,
            Stopping,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ServiceWorkerVersionStatus {
            New,
            Installing,
            Installed,
            Activating,
            Activated,
            Redundant,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ServiceWorkerRegistration {
            pub registration_id: RegistrationID,
            #[serde(default)]
            pub scope_url: String,
            #[serde(default)]
            pub is_deleted: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ServiceWorkerVersion {
            #[serde(default)]
            pub version_id: String,
            pub registration_id: RegistrationID,
            #[serde(default)]
            pub script_url: String,
            pub running_status: ServiceWorkerVersionRunningStatus,
            pub status: ServiceWorkerVersionStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub script_last_modified: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub script_response_time: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub controlled_clients: Option<Vec<Target::TargetID>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub target_id: Option<Target::TargetID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ServiceWorkerErrorMessage {
            #[serde(default)]
            pub error_message: String,
            pub registration_id: RegistrationID,
            #[serde(default)]
            pub version_id: String,
            #[serde(default)]
            pub source_url: String,
            #[serde(default)]
            pub line_number: JsUInt,
            #[serde(default)]
            pub column_number: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeliverPushMessage {
            #[serde(default)]
            pub origin: String,
            pub registration_id: RegistrationID,
            #[serde(default)]
            pub data: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchSyncEvent {
            #[serde(default)]
            pub origin: String,
            pub registration_id: RegistrationID,
            #[serde(default)]
            pub tag: String,
            #[serde(default)]
            pub last_chance: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchPeriodicSyncEvent {
            #[serde(default)]
            pub origin: String,
            pub registration_id: RegistrationID,
            #[serde(default)]
            pub tag: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct InspectWorker {
            #[serde(default)]
            pub version_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetForceUpdateOnPageLoad {
            #[serde(default)]
            pub force_update_on_page_load: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SkipWaiting {
            #[serde(default)]
            pub scope_url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartWorker {
            #[serde(default)]
            pub scope_url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopAllWorkers(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopWorker {
            #[serde(default)]
            pub version_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Unregister {
            #[serde(default)]
            pub scope_url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UpdateRegistration {
            #[serde(default)]
            pub scope_url: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DeliverPushMessageReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchSyncEventReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DispatchPeriodicSyncEventReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct InspectWorkerReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetForceUpdateOnPageLoadReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SkipWaitingReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartWorkerReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopAllWorkersReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StopWorkerReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UnregisterReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UpdateRegistrationReturnObject {}
        impl Method for DeliverPushMessage {
            const NAME: &'static str = "ServiceWorker.deliverPushMessage";
            type ReturnObject = DeliverPushMessageReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "ServiceWorker.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for DispatchSyncEvent {
            const NAME: &'static str = "ServiceWorker.dispatchSyncEvent";
            type ReturnObject = DispatchSyncEventReturnObject;
        }
        impl Method for DispatchPeriodicSyncEvent {
            const NAME: &'static str = "ServiceWorker.dispatchPeriodicSyncEvent";
            type ReturnObject = DispatchPeriodicSyncEventReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "ServiceWorker.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for InspectWorker {
            const NAME: &'static str = "ServiceWorker.inspectWorker";
            type ReturnObject = InspectWorkerReturnObject;
        }
        impl Method for SetForceUpdateOnPageLoad {
            const NAME: &'static str = "ServiceWorker.setForceUpdateOnPageLoad";
            type ReturnObject = SetForceUpdateOnPageLoadReturnObject;
        }
        impl Method for SkipWaiting {
            const NAME: &'static str = "ServiceWorker.skipWaiting";
            type ReturnObject = SkipWaitingReturnObject;
        }
        impl Method for StartWorker {
            const NAME: &'static str = "ServiceWorker.startWorker";
            type ReturnObject = StartWorkerReturnObject;
        }
        impl Method for StopAllWorkers {
            const NAME: &'static str = "ServiceWorker.stopAllWorkers";
            type ReturnObject = StopAllWorkersReturnObject;
        }
        impl Method for StopWorker {
            const NAME: &'static str = "ServiceWorker.stopWorker";
            type ReturnObject = StopWorkerReturnObject;
        }
        impl Method for Unregister {
            const NAME: &'static str = "ServiceWorker.unregister";
            type ReturnObject = UnregisterReturnObject;
        }
        impl Method for UpdateRegistration {
            const NAME: &'static str = "ServiceWorker.updateRegistration";
            type ReturnObject = UpdateRegistrationReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WorkerErrorReportedEvent {
                pub params: WorkerErrorReportedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct WorkerErrorReportedEventParams {
                pub error_message: super::ServiceWorkerErrorMessage,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WorkerRegistrationUpdatedEvent {
                pub params: WorkerRegistrationUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct WorkerRegistrationUpdatedEventParams {
                pub registrations: Vec<super::ServiceWorkerRegistration>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct WorkerVersionUpdatedEvent {
                pub params: WorkerVersionUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct WorkerVersionUpdatedEventParams {
                pub versions: Vec<super::ServiceWorkerVersion>,
            }
        }
    }
    pub mod Storage {
        use super::types::*;
        use super::Browser;
        use super::Network;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum StorageType {
            Appcache,
            Cookies,
            FileSystems,
            Indexeddb,
            LocalStorage,
            ShaderCache,
            Websql,
            ServiceWorkers,
            CacheStorage,
            All,
            Other,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UsageForType {
            pub storage_Type: StorageType,
            #[serde(default)]
            pub usage: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TrustTokens {
            #[serde(default)]
            pub issuer_origin: String,
            #[serde(default)]
            pub count: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDataForOrigin {
            #[serde(default)]
            pub origin: String,
            #[serde(default)]
            pub storage_Types: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCookies {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub browser_context_id: Option<Browser::BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCookies {
            pub cookies: Network::CookieParam,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub browser_context_id: Option<Browser::BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearCookies {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub browser_context_id: Option<Browser::BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetUsageAndQuota {
            #[serde(default)]
            pub origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct OverrideQuotaForOrigin {
            #[serde(default)]
            pub origin: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub quota_size: Option<JsFloat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TrackCacheStorageForOrigin {
            #[serde(default)]
            pub origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TrackIndexedDBForOrigin {
            #[serde(default)]
            pub origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UntrackCacheStorageForOrigin {
            #[serde(default)]
            pub origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UntrackIndexedDBForOrigin {
            #[serde(default)]
            pub origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetTrustTokens(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearTrustTokens {
            #[serde(default)]
            pub issuer_origin: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearDataForOriginReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCookiesReturnObject {
            pub cookies: Network::Cookie,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetCookiesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearCookiesReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetUsageAndQuotaReturnObject {
            #[serde(default)]
            pub usage: JsFloat,
            #[serde(default)]
            pub quota: JsFloat,
            #[serde(default)]
            pub override_active: bool,
            pub usage_breakdown: Vec<UsageForType>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct OverrideQuotaForOriginReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TrackCacheStorageForOriginReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TrackIndexedDBForOriginReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UntrackCacheStorageForOriginReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UntrackIndexedDBForOriginReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetTrustTokensReturnObject {
            pub tokens: Vec<TrustTokens>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearTrustTokensReturnObject {
            #[serde(default)]
            pub did_delete_tokens: bool,
        }
        impl Method for ClearDataForOrigin {
            const NAME: &'static str = "Storage.clearDataForOrigin";
            type ReturnObject = ClearDataForOriginReturnObject;
        }
        impl Method for GetCookies {
            const NAME: &'static str = "Storage.getCookies";
            type ReturnObject = GetCookiesReturnObject;
        }
        impl Method for SetCookies {
            const NAME: &'static str = "Storage.setCookies";
            type ReturnObject = SetCookiesReturnObject;
        }
        impl Method for ClearCookies {
            const NAME: &'static str = "Storage.clearCookies";
            type ReturnObject = ClearCookiesReturnObject;
        }
        impl Method for GetUsageAndQuota {
            const NAME: &'static str = "Storage.getUsageAndQuota";
            type ReturnObject = GetUsageAndQuotaReturnObject;
        }
        impl Method for OverrideQuotaForOrigin {
            const NAME: &'static str = "Storage.overrideQuotaForOrigin";
            type ReturnObject = OverrideQuotaForOriginReturnObject;
        }
        impl Method for TrackCacheStorageForOrigin {
            const NAME: &'static str = "Storage.trackCacheStorageForOrigin";
            type ReturnObject = TrackCacheStorageForOriginReturnObject;
        }
        impl Method for TrackIndexedDBForOrigin {
            const NAME: &'static str = "Storage.trackIndexedDBForOrigin";
            type ReturnObject = TrackIndexedDBForOriginReturnObject;
        }
        impl Method for UntrackCacheStorageForOrigin {
            const NAME: &'static str = "Storage.untrackCacheStorageForOrigin";
            type ReturnObject = UntrackCacheStorageForOriginReturnObject;
        }
        impl Method for UntrackIndexedDBForOrigin {
            const NAME: &'static str = "Storage.untrackIndexedDBForOrigin";
            type ReturnObject = UntrackIndexedDBForOriginReturnObject;
        }
        impl Method for GetTrustTokens {
            const NAME: &'static str = "Storage.getTrustTokens";
            type ReturnObject = GetTrustTokensReturnObject;
        }
        impl Method for ClearTrustTokens {
            const NAME: &'static str = "Storage.clearTrustTokens";
            type ReturnObject = ClearTrustTokensReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct CacheStorageContentUpdatedEvent {
                pub params: CacheStorageContentUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct CacheStorageContentUpdatedEventParams {
                #[serde(default)]
                pub origin: String,
                #[serde(default)]
                pub cache_name: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct CacheStorageListUpdatedEvent {
                pub params: CacheStorageListUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct CacheStorageListUpdatedEventParams {
                #[serde(default)]
                pub origin: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct IndexedDBContentUpdatedEvent {
                pub params: IndexedDBContentUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct IndexedDBContentUpdatedEventParams {
                #[serde(default)]
                pub origin: String,
                #[serde(default)]
                pub database_name: String,
                #[serde(default)]
                pub object_store_name: String,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct IndexedDBListUpdatedEvent {
                pub params: IndexedDBListUpdatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct IndexedDBListUpdatedEventParams {
                #[serde(default)]
                pub origin: String,
            }
        }
    }
    pub mod SystemInfo {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum SubsamplingFormat {
            Yuv420,
            Yuv422,
            Yuv444,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ImageType {
            Jpeg,
            Webp,
            Unknown,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GPUDevice {
            #[serde(default)]
            pub vendor_id: JsFloat,
            #[serde(default)]
            pub device_id: JsFloat,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub sub_sys_id: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub revision: Option<JsFloat>,
            #[serde(default)]
            pub vendor_string: String,
            #[serde(default)]
            pub device_string: String,
            #[serde(default)]
            pub driver_vendor: String,
            #[serde(default)]
            pub driver_version: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Size {
            #[serde(default)]
            pub width: JsUInt,
            #[serde(default)]
            pub height: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct VideoDecodeAcceleratorCapability {
            #[serde(default)]
            pub profile: String,
            pub max_resolution: Size,
            pub min_resolution: Size,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct VideoEncodeAcceleratorCapability {
            #[serde(default)]
            pub profile: String,
            pub max_resolution: Size,
            #[serde(default)]
            pub max_framerate_numerator: JsUInt,
            #[serde(default)]
            pub max_framerate_denominator: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ImageDecodeAcceleratorCapability {
            pub image_Type: ImageType,
            pub max_dimensions: Size,
            pub min_dimensions: Size,
            pub subsamplings: Vec<SubsamplingFormat>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GPUInfo {
            pub devices: Vec<GPUDevice>,
            #[serde(default)]
            pub driver_bug_workarounds: Vec<String>,
            pub video_decoding: Vec<VideoDecodeAcceleratorCapability>,
            pub video_encoding: Vec<VideoEncodeAcceleratorCapability>,
            pub image_decoding: Vec<ImageDecodeAcceleratorCapability>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ProcessInfo {
            #[serde(default)]
            pub Type: String,
            #[serde(default)]
            pub id: JsUInt,
            #[serde(default)]
            pub cpu_time: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetInfo(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetProcessInfo(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetInfoReturnObject {
            pub gpu: GPUInfo,
            #[serde(default)]
            pub model_name: String,
            #[serde(default)]
            pub model_version: String,
            #[serde(default)]
            pub command_line: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetProcessInfoReturnObject {
            pub process_info: Vec<ProcessInfo>,
        }
        impl Method for GetInfo {
            const NAME: &'static str = "SystemInfo.getInfo";
            type ReturnObject = GetInfoReturnObject;
        }
        impl Method for GetProcessInfo {
            const NAME: &'static str = "SystemInfo.getProcessInfo";
            type ReturnObject = GetProcessInfoReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod Target {
        use super::types::*;
        use super::Browser;
        use super::Page;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type TargetID = String;
        pub type SessionID = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TargetInfo {
            pub target_id: TargetID,
            #[serde(default)]
            pub Type: String,
            #[serde(default)]
            pub title: String,
            #[serde(default)]
            pub url: String,
            #[serde(default)]
            pub attached: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub opener_id: Option<TargetID>,
            #[serde(default)]
            pub can_access_opener: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub opener_frame_id: Option<Page::FrameId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub browser_context_id: Option<Browser::BrowserContextID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoteLocation {
            #[serde(default)]
            pub host: String,
            #[serde(default)]
            pub port: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ActivateTarget {
            pub target_id: TargetID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AttachToTarget {
            pub target_id: TargetID,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub flatten: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AttachToBrowserTarget(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CloseTarget {
            pub target_id: TargetID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ExposeDevToolsProtocol {
            pub target_id: TargetID,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub binding_name: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CreateBrowserContext {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub dispose_on_detach: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub proxy_server: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub proxy_bypass_list: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub origins_with_universal_network_access: Option<Vec<String>>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBrowserContexts(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CreateTarget {
            #[serde(default)]
            pub url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub width: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub height: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub browser_context_id: Option<Browser::BrowserContextID>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub enable_begin_frame_control: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub new_window: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub background: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DetachFromTarget {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub session_id: Option<SessionID>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub target_id: Option<TargetID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisposeBrowserContext {
            pub browser_context_id: Browser::BrowserContextID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetTargetInfo {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub target_id: Option<TargetID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetTargets(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SendMessageToTarget {
            #[serde(default)]
            pub message: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub session_id: Option<SessionID>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub target_id: Option<TargetID>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAutoAttach {
            #[serde(default)]
            pub auto_attach: bool,
            #[serde(default)]
            pub wait_for_debugger_on_start: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub flatten: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AutoAttachRelated {
            pub target_id: TargetID,
            #[serde(default)]
            pub wait_for_debugger_on_start: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDiscoverTargets {
            #[serde(default)]
            pub discover: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetRemoteLocations {
            pub locations: Vec<RemoteLocation>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ActivateTargetReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AttachToTargetReturnObject {
            pub session_id: SessionID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AttachToBrowserTargetReturnObject {
            pub session_id: SessionID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CloseTargetReturnObject {
            #[serde(default)]
            pub success: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ExposeDevToolsProtocolReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CreateBrowserContextReturnObject {
            pub browser_context_id: Browser::BrowserContextID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetBrowserContextsReturnObject {
            pub browser_context_ids: Browser::BrowserContextID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct CreateTargetReturnObject {
            pub target_id: TargetID,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DetachFromTargetReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisposeBrowserContextReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetTargetInfoReturnObject {
            pub target_info: TargetInfo,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetTargetsReturnObject {
            pub target_infos: Vec<TargetInfo>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SendMessageToTargetReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAutoAttachReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AutoAttachRelatedReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetDiscoverTargetsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetRemoteLocationsReturnObject {}
        impl Method for ActivateTarget {
            const NAME: &'static str = "Target.activateTarget";
            type ReturnObject = ActivateTargetReturnObject;
        }
        impl Method for AttachToTarget {
            const NAME: &'static str = "Target.attachToTarget";
            type ReturnObject = AttachToTargetReturnObject;
        }
        impl Method for AttachToBrowserTarget {
            const NAME: &'static str = "Target.attachToBrowserTarget";
            type ReturnObject = AttachToBrowserTargetReturnObject;
        }
        impl Method for CloseTarget {
            const NAME: &'static str = "Target.closeTarget";
            type ReturnObject = CloseTargetReturnObject;
        }
        impl Method for ExposeDevToolsProtocol {
            const NAME: &'static str = "Target.exposeDevToolsProtocol";
            type ReturnObject = ExposeDevToolsProtocolReturnObject;
        }
        impl Method for CreateBrowserContext {
            const NAME: &'static str = "Target.createBrowserContext";
            type ReturnObject = CreateBrowserContextReturnObject;
        }
        impl Method for GetBrowserContexts {
            const NAME: &'static str = "Target.getBrowserContexts";
            type ReturnObject = GetBrowserContextsReturnObject;
        }
        impl Method for CreateTarget {
            const NAME: &'static str = "Target.createTarget";
            type ReturnObject = CreateTargetReturnObject;
        }
        impl Method for DetachFromTarget {
            const NAME: &'static str = "Target.detachFromTarget";
            type ReturnObject = DetachFromTargetReturnObject;
        }
        impl Method for DisposeBrowserContext {
            const NAME: &'static str = "Target.disposeBrowserContext";
            type ReturnObject = DisposeBrowserContextReturnObject;
        }
        impl Method for GetTargetInfo {
            const NAME: &'static str = "Target.getTargetInfo";
            type ReturnObject = GetTargetInfoReturnObject;
        }
        impl Method for GetTargets {
            const NAME: &'static str = "Target.getTargets";
            type ReturnObject = GetTargetsReturnObject;
        }
        impl Method for SendMessageToTarget {
            const NAME: &'static str = "Target.sendMessageToTarget";
            type ReturnObject = SendMessageToTargetReturnObject;
        }
        impl Method for SetAutoAttach {
            const NAME: &'static str = "Target.setAutoAttach";
            type ReturnObject = SetAutoAttachReturnObject;
        }
        impl Method for AutoAttachRelated {
            const NAME: &'static str = "Target.autoAttachRelated";
            type ReturnObject = AutoAttachRelatedReturnObject;
        }
        impl Method for SetDiscoverTargets {
            const NAME: &'static str = "Target.setDiscoverTargets";
            type ReturnObject = SetDiscoverTargetsReturnObject;
        }
        impl Method for SetRemoteLocations {
            const NAME: &'static str = "Target.setRemoteLocations";
            type ReturnObject = SetRemoteLocationsReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AttachedToTargetEvent {
                pub params: AttachedToTargetEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AttachedToTargetEventParams {
                pub session_id: super::SessionID,
                pub target_info: super::TargetInfo,
                #[serde(default)]
                pub waiting_for_debugger: bool,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DetachedFromTargetEvent {
                pub params: DetachedFromTargetEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DetachedFromTargetEventParams {
                pub session_id: super::SessionID,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub target_id: Option<super::TargetID>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ReceivedMessageFromTargetEvent {
                pub params: ReceivedMessageFromTargetEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ReceivedMessageFromTargetEventParams {
                pub session_id: super::SessionID,
                #[serde(default)]
                pub message: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub target_id: Option<super::TargetID>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TargetCreatedEvent {
                pub params: TargetCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct TargetCreatedEventParams {
                pub target_info: super::TargetInfo,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TargetDestroyedEvent {
                pub params: TargetDestroyedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct TargetDestroyedEventParams {
                pub target_id: super::TargetID,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TargetCrashedEvent {
                pub params: TargetCrashedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct TargetCrashedEventParams {
                pub target_id: super::TargetID,
                #[serde(default)]
                pub status: String,
                #[serde(default)]
                pub error_code: JsUInt,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TargetInfoChangedEvent {
                pub params: TargetInfoChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct TargetInfoChangedEventParams {
                pub target_info: super::TargetInfo,
            }
        }
    }
    pub mod Tethering {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Bind {
            #[serde(default)]
            pub port: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Unbind {
            #[serde(default)]
            pub port: JsUInt,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BindReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct UnbindReturnObject {}
        impl Method for Bind {
            const NAME: &'static str = "Tethering.bind";
            type ReturnObject = BindReturnObject;
        }
        impl Method for Unbind {
            const NAME: &'static str = "Tethering.unbind";
            type ReturnObject = UnbindReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AcceptedEvent {
                pub params: AcceptedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AcceptedEventParams {
                #[serde(default)]
                pub port: JsUInt,
                #[serde(default)]
                pub connection_id: String,
            }
        }
    }
    pub mod Tracing {
        use super::types::*;
        use super::IO;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum TraceConfigRecordMode {
            RecordUntilFull,
            RecordContinuously,
            RecordAsMuchAsPossible,
            EchoToConsole,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum StreamFormat {
            Json,
            Proto,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum StreamCompression {
            None,
            Gzip,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum MemoryDumpLevelOfDetail {
            Background,
            Light,
            Detailed,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum TracingBackend {
            Auto,
            Chrome,
            System,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum StartTransfer_modeOption {
            ReportEvents,
            ReturnAsStream,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct MemoryDumpConfig(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TraceConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub record_mode: Option<TraceConfigRecordMode>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub enable_sampling: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub enable_systrace: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub enable_argument_filter: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub included_categories: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub excluded_categories: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub synthetic_delays: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub memory_dump_config: Option<MemoryDumpConfig>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct End(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCategories(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RecordClockSyncMarker {
            #[serde(default)]
            pub sync_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestMemoryDump {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub deterministic: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub level_of_detail: Option<MemoryDumpLevelOfDetail>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Start {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub categories: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub options: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub buffer_usage_reporting_interval: Option<JsFloat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub transfer_mode: Option<StartTransfer_modeOption>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stream_format: Option<StreamFormat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub stream_compression: Option<StreamCompression>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub trace_config: Option<TraceConfig>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub perfetto_config: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub tracing_backend: Option<TracingBackend>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EndReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCategoriesReturnObject {
            pub categories: Vec<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RecordClockSyncMarkerReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestMemoryDumpReturnObject {
            #[serde(default)]
            pub dump_guid: String,
            #[serde(default)]
            pub success: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct StartReturnObject {}
        impl Method for End {
            const NAME: &'static str = "Tracing.end";
            type ReturnObject = EndReturnObject;
        }
        impl Method for GetCategories {
            const NAME: &'static str = "Tracing.getCategories";
            type ReturnObject = GetCategoriesReturnObject;
        }
        impl Method for RecordClockSyncMarker {
            const NAME: &'static str = "Tracing.recordClockSyncMarker";
            type ReturnObject = RecordClockSyncMarkerReturnObject;
        }
        impl Method for RequestMemoryDump {
            const NAME: &'static str = "Tracing.requestMemoryDump";
            type ReturnObject = RequestMemoryDumpReturnObject;
        }
        impl Method for Start {
            const NAME: &'static str = "Tracing.start";
            type ReturnObject = StartReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct BufferUsageEvent {
                pub params: BufferUsageEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct BufferUsageEventParams {
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub percent_full: Option<JsFloat>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub event_count: Option<JsFloat>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub value: Option<JsFloat>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct DataCollectedEvent {
                pub params: DataCollectedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct DataCollectedEventParams {}
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct TracingCompleteEvent {
                pub params: TracingCompleteEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct TracingCompleteEventParams {
                #[serde(default)]
                pub data_loss_occurred: bool,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub stream: Option<super::super::IO::StreamHandle>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub trace_format: Option<super::StreamFormat>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub stream_compression: Option<super::StreamCompression>,
            }
        }
    }
    pub mod Fetch {
        use super::types::*;
        use super::Network;
        use super::Page;
        use super::IO;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type RequestId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum RequestStage {
            Request,
            Response,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum AuthChallengeSource {
            Server,
            Proxy,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "PascalCase")]
        pub enum AuthChallengeResponseResponse {
            Default,
            CancelAuth,
            ProvideCredentials,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RequestPattern {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url_pattern: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub resource_Type: Option<Network::ResourceType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub request_stage: Option<RequestStage>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct HeaderEntry {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthChallenge {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub source: Option<AuthChallengeSource>,
            #[serde(default)]
            pub origin: String,
            #[serde(default)]
            pub scheme: String,
            #[serde(default)]
            pub realm: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthChallengeResponse {
            pub response: AuthChallengeResponseResponse,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub username: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub password: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub patterns: Option<Vec<RequestPattern>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub handle_auth_requests: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FailRequest {
            pub request_id: RequestId,
            pub error_reason: Network::ErrorReason,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FulfillRequest {
            pub request_id: RequestId,
            #[serde(default)]
            pub response_code: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub response_headers: Option<Vec<HeaderEntry>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub binary_response_headers: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub body: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub response_phrase: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContinueRequest {
            pub request_id: RequestId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub method: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub post_data: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub headers: Option<Vec<HeaderEntry>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub intercept_response: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContinueWithAuth {
            pub request_id: RequestId,
            pub auth_challenge_response: AuthChallengeResponse,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContinueResponse {
            pub request_id: RequestId,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub response_code: Option<JsUInt>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub response_phrase: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub response_headers: Option<Vec<HeaderEntry>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub binary_response_headers: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetResponseBody {
            pub request_id: RequestId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeResponseBodyAsStream {
            pub request_id: RequestId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FailRequestReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct FulfillRequestReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContinueRequestReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContinueWithAuthReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContinueResponseReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetResponseBodyReturnObject {
            #[serde(default)]
            pub body: String,
            #[serde(default)]
            pub base_64_encoded: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct TakeResponseBodyAsStreamReturnObject {
            pub stream: IO::StreamHandle,
        }
        impl Method for Disable {
            const NAME: &'static str = "Fetch.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for Enable {
            const NAME: &'static str = "Fetch.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for FailRequest {
            const NAME: &'static str = "Fetch.failRequest";
            type ReturnObject = FailRequestReturnObject;
        }
        impl Method for FulfillRequest {
            const NAME: &'static str = "Fetch.fulfillRequest";
            type ReturnObject = FulfillRequestReturnObject;
        }
        impl Method for ContinueRequest {
            const NAME: &'static str = "Fetch.continueRequest";
            type ReturnObject = ContinueRequestReturnObject;
        }
        impl Method for ContinueWithAuth {
            const NAME: &'static str = "Fetch.continueWithAuth";
            type ReturnObject = ContinueWithAuthReturnObject;
        }
        impl Method for ContinueResponse {
            const NAME: &'static str = "Fetch.continueResponse";
            type ReturnObject = ContinueResponseReturnObject;
        }
        impl Method for GetResponseBody {
            const NAME: &'static str = "Fetch.getResponseBody";
            type ReturnObject = GetResponseBodyReturnObject;
        }
        impl Method for TakeResponseBodyAsStream {
            const NAME: &'static str = "Fetch.takeResponseBodyAsStream";
            type ReturnObject = TakeResponseBodyAsStreamReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct RequestPausedEvent {
                pub params: RequestPausedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct RequestPausedEventParams {
                pub request_id: super::RequestId,
                pub request: super::super::Network::Request,
                pub frame_id: super::super::Page::FrameId,
                pub resource_Type: super::super::Network::ResourceType,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub response_error_reason: Option<super::super::Network::ErrorReason>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub response_status_code: Option<JsUInt>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub response_status_text: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub response_headers: Option<Vec<super::HeaderEntry>>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub network_id: Option<super::RequestId>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AuthRequiredEvent {
                pub params: AuthRequiredEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AuthRequiredEventParams {
                pub request_id: super::RequestId,
                pub request: super::super::Network::Request,
                pub frame_id: super::super::Page::FrameId,
                pub resource_Type: super::super::Network::ResourceType,
                pub auth_challenge: super::AuthChallenge,
            }
        }
    }
    pub mod WebAudio {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type GraphObjectId = String;
        pub type NodeType = String;
        pub type ParamType = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ContextType {
            Realtime,
            Offline,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ContextState {
            Suspended,
            Running,
            Closed,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum ChannelCountMode {
            ClampedMax,
            Explicit,
            Max,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum ChannelInterpretation {
            Discrete,
            Speakers,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "kebab-case")]
        pub enum AutomationRate {
            ARate,
            KRate,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ContextRealtimeData {
            #[serde(default)]
            pub current_time: JsFloat,
            #[serde(default)]
            pub render_capacity: JsFloat,
            #[serde(default)]
            pub callback_interval_mean: JsFloat,
            #[serde(default)]
            pub callback_interval_variance: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct BaseAudioContext {
            pub context_id: GraphObjectId,
            pub context_Type: ContextType,
            pub context_state: ContextState,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub realtime_data: Option<ContextRealtimeData>,
            #[serde(default)]
            pub callback_buffer_size: JsFloat,
            #[serde(default)]
            pub max_output_channel_count: JsFloat,
            #[serde(default)]
            pub sample_rate: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AudioListener {
            pub listener_id: GraphObjectId,
            pub context_id: GraphObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AudioNode {
            pub node_id: GraphObjectId,
            pub context_id: GraphObjectId,
            pub node_Type: NodeType,
            #[serde(default)]
            pub number_of_inputs: JsFloat,
            #[serde(default)]
            pub number_of_outputs: JsFloat,
            #[serde(default)]
            pub channel_count: JsFloat,
            pub channel_count_mode: ChannelCountMode,
            pub channel_interpretation: ChannelInterpretation,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AudioParam {
            pub param_id: GraphObjectId,
            pub node_id: GraphObjectId,
            pub context_id: GraphObjectId,
            pub param_Type: ParamType,
            pub rate: AutomationRate,
            #[serde(default)]
            pub default_value: JsFloat,
            #[serde(default)]
            pub min_value: JsFloat,
            #[serde(default)]
            pub max_value: JsFloat,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetRealtimeData {
            pub context_id: GraphObjectId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetRealtimeDataReturnObject {
            pub realtime_data: ContextRealtimeData,
        }
        impl Method for Enable {
            const NAME: &'static str = "WebAudio.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "WebAudio.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for GetRealtimeData {
            const NAME: &'static str = "WebAudio.getRealtimeData";
            type ReturnObject = GetRealtimeDataReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ContextCreatedEvent {
                pub params: ContextCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ContextCreatedEventParams {
                pub context: super::BaseAudioContext,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ContextWillBeDestroyedEvent {
                pub params: ContextWillBeDestroyedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ContextWillBeDestroyedEventParams {
                pub context_id: super::GraphObjectId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct ContextChangedEvent {
                pub params: ContextChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ContextChangedEventParams {
                pub context: super::BaseAudioContext,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioListenerCreatedEvent {
                pub params: AudioListenerCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AudioListenerCreatedEventParams {
                pub listener: super::AudioListener,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioListenerWillBeDestroyedEvent {
                pub params: AudioListenerWillBeDestroyedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AudioListenerWillBeDestroyedEventParams {
                pub context_id: super::GraphObjectId,
                pub listener_id: super::GraphObjectId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioNodeCreatedEvent {
                pub params: AudioNodeCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AudioNodeCreatedEventParams {
                pub node: super::AudioNode,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioNodeWillBeDestroyedEvent {
                pub params: AudioNodeWillBeDestroyedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AudioNodeWillBeDestroyedEventParams {
                pub context_id: super::GraphObjectId,
                pub node_id: super::GraphObjectId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioParamCreatedEvent {
                pub params: AudioParamCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AudioParamCreatedEventParams {
                pub param: super::AudioParam,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct AudioParamWillBeDestroyedEvent {
                pub params: AudioParamWillBeDestroyedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AudioParamWillBeDestroyedEventParams {
                pub context_id: super::GraphObjectId,
                pub node_id: super::GraphObjectId,
                pub param_id: super::GraphObjectId,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodesConnectedEvent {
                pub params: NodesConnectedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct NodesConnectedEventParams {
                pub context_id: super::GraphObjectId,
                pub source_id: super::GraphObjectId,
                pub destination_id: super::GraphObjectId,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub source_output_index: Option<JsFloat>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub destination_input_index: Option<JsFloat>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodesDisconnectedEvent {
                pub params: NodesDisconnectedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct NodesDisconnectedEventParams {
                pub context_id: super::GraphObjectId,
                pub source_id: super::GraphObjectId,
                pub destination_id: super::GraphObjectId,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub source_output_index: Option<JsFloat>,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub destination_input_index: Option<JsFloat>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodeParamConnectedEvent {
                pub params: NodeParamConnectedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct NodeParamConnectedEventParams {
                pub context_id: super::GraphObjectId,
                pub source_id: super::GraphObjectId,
                pub destination_id: super::GraphObjectId,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub source_output_index: Option<JsFloat>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct NodeParamDisconnectedEvent {
                pub params: NodeParamDisconnectedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct NodeParamDisconnectedEventParams {
                pub context_id: super::GraphObjectId,
                pub source_id: super::GraphObjectId,
                pub destination_id: super::GraphObjectId,
                #[serde(skip_serializing_if = "Option::is_none")]
                #[serde(default)]
                pub source_output_index: Option<JsFloat>,
            }
        }
    }
    pub mod WebAuthn {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type AuthenticatorId = String;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum AuthenticatorProtocol {
            U2F,
            Ctap2,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum Ctap2Version {
            Ctap20,
            Ctap21,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum AuthenticatorTransport {
            Usb,
            Nfc,
            Ble,
            Cable,
            Internal,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct VirtualAuthenticatorOptions {
            pub protocol: AuthenticatorProtocol,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ctap_2_version: Option<Ctap2Version>,
            pub transport: AuthenticatorTransport,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub has_resident_key: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub has_user_verification: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub has_large_blob: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub has_cred_blob: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub automatic_presence_simulation: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub is_user_verified: Option<bool>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Credential {
            #[serde(default)]
            pub credential_id: String,
            #[serde(default)]
            pub is_resident_credential: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub rp_id: Option<String>,
            #[serde(default)]
            pub private_key: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub user_handle: Option<String>,
            #[serde(default)]
            pub sign_count: JsUInt,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            pub large_blob: Option<String>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddVirtualAuthenticator {
            pub options: VirtualAuthenticatorOptions,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveVirtualAuthenticator {
            pub authenticator_id: AuthenticatorId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddCredential {
            pub authenticator_id: AuthenticatorId,
            pub credential: Credential,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCredential {
            pub authenticator_id: AuthenticatorId,
            #[serde(default)]
            pub credential_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCredentials {
            pub authenticator_id: AuthenticatorId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveCredential {
            pub authenticator_id: AuthenticatorId,
            #[serde(default)]
            pub credential_id: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearCredentials {
            pub authenticator_id: AuthenticatorId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetUserVerified {
            pub authenticator_id: AuthenticatorId,
            #[serde(default)]
            pub is_user_verified: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAutomaticPresenceSimulation {
            pub authenticator_id: AuthenticatorId,
            #[serde(default)]
            pub enabled: bool,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddVirtualAuthenticatorReturnObject {
            pub authenticator_id: AuthenticatorId,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveVirtualAuthenticatorReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct AddCredentialReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCredentialReturnObject {
            pub credential: Credential,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct GetCredentialsReturnObject {
            pub credentials: Vec<Credential>,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct RemoveCredentialReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct ClearCredentialsReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetUserVerifiedReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct SetAutomaticPresenceSimulationReturnObject {}
        impl Method for Enable {
            const NAME: &'static str = "WebAuthn.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "WebAuthn.disable";
            type ReturnObject = DisableReturnObject;
        }
        impl Method for AddVirtualAuthenticator {
            const NAME: &'static str = "WebAuthn.addVirtualAuthenticator";
            type ReturnObject = AddVirtualAuthenticatorReturnObject;
        }
        impl Method for RemoveVirtualAuthenticator {
            const NAME: &'static str = "WebAuthn.removeVirtualAuthenticator";
            type ReturnObject = RemoveVirtualAuthenticatorReturnObject;
        }
        impl Method for AddCredential {
            const NAME: &'static str = "WebAuthn.addCredential";
            type ReturnObject = AddCredentialReturnObject;
        }
        impl Method for GetCredential {
            const NAME: &'static str = "WebAuthn.getCredential";
            type ReturnObject = GetCredentialReturnObject;
        }
        impl Method for GetCredentials {
            const NAME: &'static str = "WebAuthn.getCredentials";
            type ReturnObject = GetCredentialsReturnObject;
        }
        impl Method for RemoveCredential {
            const NAME: &'static str = "WebAuthn.removeCredential";
            type ReturnObject = RemoveCredentialReturnObject;
        }
        impl Method for ClearCredentials {
            const NAME: &'static str = "WebAuthn.clearCredentials";
            type ReturnObject = ClearCredentialsReturnObject;
        }
        impl Method for SetUserVerified {
            const NAME: &'static str = "WebAuthn.setUserVerified";
            type ReturnObject = SetUserVerifiedReturnObject;
        }
        impl Method for SetAutomaticPresenceSimulation {
            const NAME: &'static str = "WebAuthn.setAutomaticPresenceSimulation";
            type ReturnObject = SetAutomaticPresenceSimulationReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
        }
    }
    pub mod Media {
        use super::types::*;
        use serde::{Deserialize, Serialize};
        use serde_json::Value as Json;
        pub type PlayerId = String;
        pub type Timestamp = JsFloat;
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum PlayerMessageLevel {
            Error,
            Warning,
            Info,
            Debug,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub enum PlayerErrorType {
            PipelineError,
            MediaError,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PlayerMessage {
            pub level: PlayerMessageLevel,
            #[serde(default)]
            pub message: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PlayerProperty {
            #[serde(default)]
            pub name: String,
            #[serde(default)]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PlayerEvent {
            pub timestamp: Timestamp,
            #[serde(default)]
            pub value: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct PlayerError {
            pub Type: PlayerErrorType,
            #[serde(default)]
            pub error_code: String,
        }
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Enable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Disable(pub Option<serde_json::Value>);
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct EnableReturnObject {}
        #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct DisableReturnObject {}
        impl Method for Enable {
            const NAME: &'static str = "Media.enable";
            type ReturnObject = EnableReturnObject;
        }
        impl Method for Disable {
            const NAME: &'static str = "Media.disable";
            type ReturnObject = DisableReturnObject;
        }
        pub mod events {
            use super::super::types::*;
            use serde::{Deserialize, Serialize};
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PlayerPropertiesChangedEvent {
                pub params: PlayerPropertiesChangedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct PlayerPropertiesChangedEventParams {
                pub player_id: super::PlayerId,
                pub properties: Vec<super::PlayerProperty>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PlayerEventsAddedEvent {
                pub params: PlayerEventsAddedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct PlayerEventsAddedEventParams {
                pub player_id: super::PlayerId,
                pub events: Vec<super::PlayerEvent>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PlayerMessagesLoggedEvent {
                pub params: PlayerMessagesLoggedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct PlayerMessagesLoggedEventParams {
                pub player_id: super::PlayerId,
                pub messages: Vec<super::PlayerMessage>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PlayerErrorsRaisedEvent {
                pub params: PlayerErrorsRaisedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct PlayerErrorsRaisedEventParams {
                pub player_id: super::PlayerId,
                pub errors: Vec<super::PlayerError>,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            pub struct PlayersCreatedEvent {
                pub params: PlayersCreatedEventParams,
            }
            #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct PlayersCreatedEventParams {
                pub players: Vec<super::PlayerId>,
            }
        }
    }
}
